use crate::{
    encoding::deserialize_p2s_to_ergo_tree,
    encoding::{serialize_address_from_ergo_tree, serialize_p2s_from_ergo_tree},
    ErgsBox, NanoErg,
};
use ergo_lib::{
    chain::ergo_box::{BoxValue, ErgoBox, NonMandatoryRegisters},
    chain::transaction::unsigned::UnsignedTransaction,
    chain::transaction::TxId,
};
use ergo_lib_wasm::transaction::UnsignedTransaction as WUnsignedTransaction;
use json::object;
use wasm_bindgen::prelude::*;

/// This is a struct which is used to generate Ergo Tx Assembler Spec `String`s
/// from `UnsignedTransaction`s.
#[wasm_bindgen]
pub struct TxAssemblerSpecBuilder {
    unsigned_tx: UnsignedTransaction,
}

#[wasm_bindgen]
impl TxAssemblerSpecBuilder {
    /// WASM wrapper for `new()`
    #[wasm_bindgen]
    pub fn w_new(wrapped_unsigned_tx: WUnsignedTransaction) -> Self {
        let unsigned_tx = wrapped_unsigned_tx.into();
        Self::new(unsigned_tx)
    }

    /// Create a placeholder box that holds an amount of nanoErgs equal to the
    /// input `nano_ergs` value and then wrap said box as a `ErgsBox`.
    /// This is useful for using with protocols as a placeholder so that
    /// an assembler spec can be created (and this placeholder box thrown out
    /// and replaced with the user's actual input box from the assembler)
    #[wasm_bindgen]
    pub fn create_placeholder_ergs_box(nano_ergs: NanoErg) -> Option<ErgsBox> {
        let placeholder_address = "2iHkR7CWvD1R4j1yZg5bkeDRQavjAaVPeTDFGGLZduHyfWMuYpmhHocX8GJoaieTx78FntzJbCBVL6rf96ocJoZdmWBL2fci7NqWgAirppPQmZ7fN9V6z13Ay6brPriBKYqLp1bT2Fk4FkFLCfdPpe".to_string();
        let ergo_tree = deserialize_p2s_to_ergo_tree(placeholder_address).ok()?;
        let box_value = BoxValue::new(nano_ergs).ok()?;
        let placeholder_box = ErgoBox::new(
            box_value,
            ergo_tree,
            vec![],
            NonMandatoryRegisters::empty(),
            0,
            TxId::zero(),
            0,
        );

        ErgsBox::new(&placeholder_box).ok()
    }

    /// Builds a JSON `String` which
    /// is formatted as a transaction spec for working with the
    /// Ergo Transaction Assembler Service.
    #[wasm_bindgen]
    pub fn build_assembler_spec(&self, transaction_fee: NanoErg) -> String {
        let mut tx_spec = object! {
            "requests":  [],
        };

        for i in 0..self.unsigned_tx.output_candidates.len() {
            let output = self.unsigned_tx.output_candidates[i].clone();
            // Base values
            tx_spec["requests"][i]["value"] = output.value.as_u64().clone().into();
            if let Ok(address_string) = serialize_address_from_ergo_tree(output.ergo_tree.clone()) {
                tx_spec["requests"][i]["address"] = address_string.into();
            } else {
                tx_spec["requests"][i]["address"] =
                    serialize_p2s_from_ergo_tree(output.ergo_tree).into();
            }

            // Tokens
            for n in 0..output.tokens.len() {
                let token = output.tokens[n].clone();
                let tok_id: String = token.token_id.0.into();
                let tok_amount: u64 = token.amount.into();
                tx_spec["requests"][i]["assets"][n]["tokenId"] = tok_id.into();
                tx_spec["requests"][i]["assets"][n]["amount"] = tok_amount.into();
            }

            // Registers
            let registers = output.additional_registers.get_ordered_values();
            for y in 0..registers.len() {
                if y == 0 {
                    tx_spec["requests"][i]["registers"]["R4"] = registers[y].base16_str().into();
                }
                if y == 1 {
                    tx_spec["requests"][i]["registers"]["R5"] = registers[y].base16_str().into();
                }
                if y == 2 {
                    tx_spec["requests"][i]["registers"]["R6"] = registers[y].base16_str().into();
                }
                if y == 3 {
                    tx_spec["requests"][i]["registers"]["R7"] = registers[y].base16_str().into();
                }
                if y == 4 {
                    tx_spec["requests"][i]["registers"]["R8"] = registers[y].base16_str().into();
                }
                if y == 5 {
                    tx_spec["requests"][i]["registers"]["R9"] = registers[y].base16_str().into();
                }
            }

            // Fee
            tx_spec["fee"] = transaction_fee.into();

            // Inputs
            let inputs = self.unsigned_tx.inputs.clone();
            for y in 0..inputs.len() {
                let box_id: String = inputs[y].box_id.clone().into();
                tx_spec["inputs"][y] = box_id.into();
            }

            // Inputs
            let data_inputs = self.unsigned_tx.data_inputs.clone();
            for y in 0..data_inputs.len() {
                let box_id: String = data_inputs[y].box_id.clone().into();
                tx_spec["dataInputs"][y] = box_id.into();
            }
        }

        tx_spec.pretty(2)
    }
}

/// Non-WASM methods
impl TxAssemblerSpecBuilder {
    /// Create a new `TxAssemblerSpecBuilder`
    pub fn new(unsigned_tx: UnsignedTransaction) -> Self {
        TxAssemblerSpecBuilder {
            unsigned_tx: unsigned_tx,
        }
    }
}
