// This file holds a number of functions which aid in tx creation.
use crate::error::{HeadlessDappError, Result};
use crate::{
    encoding::address_string_to_ergo_tree, encoding::serialize_p2s_from_ergo_tree,
    P2PKAddressString, P2SAddressString,
};
use crate::{BlockHeight, ErgoAddressString, NanoErg};
use ergo_lib::chain::ergo_box::{BoxValue, ErgoBox, ErgoBoxCandidate, NonMandatoryRegisters};
use ergo_lib::chain::token::{Token, TokenAmount};
use ergo_lib::{ast::constant::Constant, chain::transaction::unsigned::UnsignedTransaction};
use ergo_lib_wasm::transaction::UnsignedTransaction as WUnsignedTransaction;
use json::object;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

/// Create an `ErgoBoxCandidate`
pub fn create_candidate(
    value: NanoErg,
    output_address: &ErgoAddressString,
    tokens: &Vec<Token>,
    registers: &Vec<Constant>,
    current_height: BlockHeight,
) -> Result<ErgoBoxCandidate> {
    let obb_value = BoxValue::new(value).map_err(|_| HeadlessDappError::InvalidBoxValue(value))?;
    let obb_registers = NonMandatoryRegisters::from_ordered_values(registers.clone())
        .map_err(|_| HeadlessDappError::InvalidRegisterValues())?;
    // Obtain ErgoTree of the output_address
    let obb_ergo_tree = address_string_to_ergo_tree(output_address)
        .map_err(|_| HeadlessDappError::InvalidP2PKAddress(output_address.clone()))?;
    // Create the output Bank box candidate
    let output_bank_candidate = ErgoBoxCandidate {
        value: obb_value,
        ergo_tree: obb_ergo_tree,
        tokens: tokens.clone(),
        additional_registers: obb_registers.clone(),
        creation_height: current_height as u32,
    };
    Ok(output_bank_candidate)
}

/// Finds all tokens held by `ErgoBox`es (generally from a list of inputs),
/// which are not in the list of `filter_tokens`. Once found the tokens are
/// also summed and then returned.
pub fn find_and_sum_other_tokens(
    filter_tokens: &Vec<Token>,
    input_boxes: &Vec<ErgoBox>,
) -> Vec<Token> {
    let mut new_tokens: Vec<Token> = vec![];
    for b in input_boxes {
        for t in b.tokens.clone() {
            // If token `t` has an id of one of the known tokens
            let filter_tokens_check = filter_tokens.iter().any(|tok| tok.token_id == t.token_id);
            // If token `t` has an id of one of the already added new tokens
            let new_tokens_check = new_tokens.iter().any(|tok| t.token_id == tok.token_id);
            if filter_tokens_check {
                continue;
            } else if new_tokens_check {
                new_tokens = new_tokens
                    .iter()
                    .map(|tok| {
                        if tok.token_id == t.token_id {
                            // Replace the previous token with a new struct
                            // that has the new total amount.
                            Token {
                                token_id: tok.token_id.clone(),
                                amount: TokenAmount::try_from(
                                    u64::from(tok.amount) + u64::from(t.amount),
                                )
                                // This unwrap is safe due to the values being
                                // added originally coming from `TokenAmount`s
                                // and the TokenIDs being the same.
                                .unwrap(),
                            }
                        } else {
                            tok.clone()
                        }
                    })
                    .collect();
            } else {
                new_tokens.push(t)
            }
        }
    }
    new_tokens
}

/// Given an `UnsignedTransaction`, builds a JSON `String` which
/// is formatted as a transaction spec for working with the
/// Transaction Assembler Service.
#[wasm_bindgen]
pub fn w_unsigned_transaction_to_assembler_spec(
    wrapped_unsigned_tx: WUnsignedTransaction,
    transaction_fee: NanoErg,
) -> String {
    let unsigned_tx = wrapped_unsigned_tx.into();
    unsigned_transaction_to_assembler_spec(unsigned_tx, transaction_fee)
}

/// Given an `UnsignedTransaction`, builds a JSON `String` which
/// is formatted as a transaction spec for working with the
/// Transaction Assembler Service.
pub fn unsigned_transaction_to_assembler_spec(
    unsigned_tx: UnsignedTransaction,
    transaction_fee: NanoErg,
) -> String {
    let mut tx_spec = object! {
        "requests":  [],
    };

    for i in 0..unsigned_tx.output_candidates.len() {
        let output = unsigned_tx.output_candidates[i].clone();
        // Base values
        tx_spec["requests"][i]["value"] = output.value.as_u64().clone().into();
        tx_spec["requests"][i]["address"] = serialize_p2s_from_ergo_tree(output.ergo_tree).into();

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
        let inputs = unsigned_tx.inputs.clone();
        for y in 0..inputs.len() {
            let box_id: String = inputs[y].box_id.clone().into();
            tx_spec["inputs"][y] = box_id.into();
        }

        // Inputs
        let data_inputs = unsigned_tx.data_inputs.clone();
        for y in 0..data_inputs.len() {
            let box_id: String = data_inputs[y].box_id.clone().into();
            tx_spec["dataInputs"][y] = box_id.into();
        }
    }

    tx_spec.pretty(2)
}
