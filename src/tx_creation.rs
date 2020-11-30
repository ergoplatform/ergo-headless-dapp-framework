// This file holds a number of functions which aid in tx creation.
use crate::encoding::address_string_to_ergo_tree;
use crate::error::{HeadlessDappError, Result};
use crate::{BlockHeight, ErgoAddressString, NanoErg};
use ergo_lib::ast::constant::Constant;
use ergo_lib::chain::ergo_box::{BoxValue, ErgoBox, ErgoBoxCandidate, NonMandatoryRegisters};
use ergo_lib::chain::token::{Token, TokenAmount};
use std::convert::TryFrom;

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
