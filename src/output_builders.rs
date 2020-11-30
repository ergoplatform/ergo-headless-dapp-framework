/// Structs that provide an interface for creating `ErgoBoxCandidate`s
/// to be used during tx creation in Actions in a protocol.
/// These "Output Builders" only provide the ability to create output
/// candidates;
use crate::error::Result;
use crate::tx_creation::{create_candidate, find_and_sum_other_tokens};
use crate::{NanoErg, P2PKAddressString};
use ergo_lib::ast::constant::Constant;
use ergo_lib::chain::ergo_box::{ErgoBox, ErgoBoxCandidate};
use ergo_lib::chain::token::Token;

// /// A function which takes all input boxes/output candidates
// /// and creates two new output candidates. One tx fee box, and one
// /// change box which holds
// /// all nanoErgs/tokens which are unaccounted for in inputs/outputs.
// /// This function then builds and returns an `UnsignedTransaction`.
// pub fn balance_and_create_unsigned_tx(inputs: Vec<ErgoBox>, data_inputs: Vec<ErgoBox>, outputs: Vec<ErgoBoxCandidate>, transaction_fee: NanoErg) -> Result<ErgoBoxCandidate>

/// A struct used while constructing txs to build a `ErgoBoxCandidate`
/// that holds any change Ergs & tokens from the input boxes which are
/// not relevant to the protocol at hand
pub struct ChangeBox {}

impl ChangeBox {
    /// Creates an `ErgoBoxCandidate` which holds all of the tokens
    /// and Ergs provided as inputs to the method.
    pub fn output_candidate(
        tokens: &Vec<Token>,
        value: NanoErg,
        user_address: &P2PKAddressString,
        current_height: u64,
    ) -> Result<ErgoBoxCandidate> {
        create_candidate(value, &user_address, &tokens, &vec![], current_height)
    }
}

/// A struct used while constructing txs to build a `ErgoBoxCandidate`
/// that holds various tokens from the input boxes which are not relevant
/// to the protocol at hand. In other words a "Tokens Change Box".
pub struct TokensChangeBox {}

impl TokensChangeBox {
    /// Creates an `ErgoBoxCandidate` which holds all of the tokens from the
    /// the provided inputs. In other words creates a "TokensChange" box for
    /// the tokens inside of the inputs.
    /// Holds number of nanoErgs value as provided to method.
    pub fn output_candidate(
        input_boxes: &Vec<ErgoBox>,
        value: NanoErg,
        user_address: &P2PKAddressString,
        current_height: u64,
    ) -> Result<ErgoBoxCandidate> {
        // Find the tokens that exist in the inputs which need to be preserved
        let tc_tokens = find_and_sum_other_tokens(&vec![], input_boxes);
        create_candidate(value, &user_address, &tc_tokens, &vec![], current_height)
    }

    /// Creates an `ErgoBoxCandidate` which holds tokens from the
    /// the provided inputs excluding the tokens provided in the filter list.
    /// Holds number of nanoErgs value as provided to method.
    pub fn output_candidate_filtered(
        filter_tokens: &Vec<Token>,
        input_boxes: &Vec<ErgoBox>,
        value: NanoErg,
        user_address: &P2PKAddressString,
        current_height: u64,
    ) -> Result<ErgoBoxCandidate> {
        TokensChangeBox::output_candidate_with_registers_filtered(
            filter_tokens,
            input_boxes,
            value,
            &vec![],
            user_address,
            current_height,
        )
    }

    /// Creates an `ErgoBoxCandidate` which holds tokens from the
    /// the provided inputs excluding the tokens provided in the filter list.
    /// Holds number of nanoErgs value as provided to method and uses the
    /// customized registers provided.
    pub fn output_candidate_with_registers_filtered(
        filter_tokens: &Vec<Token>,
        input_boxes: &Vec<ErgoBox>,
        value: NanoErg,
        registers: &Vec<Constant>,
        user_address: &P2PKAddressString,
        current_height: u64,
    ) -> Result<ErgoBoxCandidate> {
        // Find the tokens that exist in the inputs which need to be preserved
        let tc_tokens = find_and_sum_other_tokens(filter_tokens, input_boxes);

        create_candidate(value, &user_address, &tc_tokens, registers, current_height)
    }
}

/// A struct used while constructing txs to build a `ErgoBoxCandidate`
/// that holds a tx fee and is sent to the miner script address.
pub struct TxFeeBox {}

impl TxFeeBox {
    pub fn output_candidate(tx_fee: u64, current_height: u64) -> Result<ErgoBoxCandidate> {
        create_candidate(
            tx_fee,
            &"2iHkR7CWvD1R4j1yZg5bkeDRQavjAaVPeTDFGGLZduHyfWMuYpmhHocX8GJoaieTx78FntzJbCBVL6rf96ocJoZdmWBL2fci7NqWgAirppPQmZ7fN9V6z13Ay6brPriBKYqLp1bT2Fk4FkFLCfdPpe".to_string(),
            &vec![],
            &vec![],
            current_height,
        )
    }
}
