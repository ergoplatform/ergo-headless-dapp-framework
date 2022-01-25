use crate::{ErgoAddressString, P2SAddressString};
use base16;
use blake2b_simd::Params;
use ergo_lib::ergotree_ir::base16_str::Base16Str;
use ergo_lib::ergotree_ir::chain::address::{Address, AddressEncoder, NetworkPrefix};
use ergo_lib::ergotree_ir::chain::base16_bytes::{Base16DecodedBytes, Base16EncodedBytes};
use ergo_lib::ergotree_ir::chain::digest32::{Digest32, Digest32Error};
use ergo_lib::ergotree_ir::chain::token::{Token, TokenAmount, TokenAmountError, TokenId};
use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use ergo_lib::ergotree_ir::mir::constant::Constant;
/// This file holds various functions related to encoding/serialization of values that are relevant
/// to the oracle core.
use ergo_lib::ergotree_ir::mir::constant::TryExtractFrom;
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::str;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, EncodingError<String>>;

#[derive(Error, Debug)]
pub enum EncodingError<T: Debug + Display> {
    #[error("Failed to serialize: {0}")]
    FailedToSerialize(T),
    #[error("Failed to deserialize: {0}")]
    FailedToDeserialize(T),
    #[error("Failed to unwrap: {0}")]
    FailedToUnwrap(T),
    #[error(transparent)]
    TokenAmountError(#[from] TokenAmountError),
    #[error(transparent)]
    Digest32Error(#[from] Digest32Error),
}

/// Takes the blake2b hash of a String, then converted into/represented as hex as a String
pub fn string_to_blake2b_hash(b: String) -> Result<String> {
    let mut params = Params::new();
    params.hash_length(32);
    let a = params.hash(&decode_hex(&b)?).to_hex().to_string();
    Ok(a)
}

/// Serialize a `String` value into a signed hex-encoded byte string
/// and then convert it into a `Constant` to be used in registers.
pub fn serialize_string(s: &String) -> Constant {
    let b = convert_to_signed_bytes(&s.clone().into_bytes());
    b.into()
}

/// Decodes a hex-encoded string into bytes and then serializes it into a properly formatted signed hex-encoded string and converted into a `Constant`
pub fn serialize_hex_encoded_string(s: &String) -> Result<Constant> {
    let b = decode_hex(s)?;
    let constant: Constant = convert_to_signed_bytes(&b).into();
    Ok(constant)
}

/// Given a P2S address, extract its `ErgoTree`, serialize it into hex-encoded
/// bytes, hash it with blake2b_256, and then prepare it to be used
/// in a register as a Constant
pub fn hash_and_serialize_p2s(address: &P2SAddressString) -> Result<Constant> {
    let ergo_tree = address_string_to_ergo_tree(&address)?;
    // Convert into hex-encoded bytes
    let base16_bytes = Base16EncodedBytes::new(&ergo_tree.sigma_serialize_bytes().unwrap());
    // Convert into String
    let ergo_tree_hex_string = base16_bytes.into();
    serialize_hex_encoded_string(&string_to_blake2b_hash(ergo_tree_hex_string)?)
}

/// Unwraps a hex-encoded `i32` Int inside of a `Constant` acquired from a register of an `ErgoBox`
pub fn unwrap_int(c: &Constant) -> Result<i32> {
    i32::try_extract_from(c.clone())
        .map_err(|_| EncodingError::FailedToUnwrap(c.base16_str().unwrap()))
}

/// Unwrap a hex-encoded `i64` Long inside of a `Constant` acquired from a register of an `ErgoBox`
pub fn unwrap_long(c: &Constant) -> Result<i64> {
    i64::try_extract_from(c.clone())
        .map_err(|_| EncodingError::FailedToUnwrap(c.base16_str().unwrap()))
}

/// Unwrap a String which is inside of a `Constant` acquired from a register of an `ErgoBox`
pub fn unwrap_string(c: &Constant) -> Result<String> {
    let byte_array: Result<Vec<u8>> = match Vec::<i8>::try_extract_from(c.clone()) {
        Ok(ba) => Ok(convert_to_unsigned_bytes(&ba)),
        _ => Err(EncodingError::FailedToUnwrap(c.base16_str().unwrap())),
    };
    Ok(str::from_utf8(&byte_array?)
        .map_err(|_| EncodingError::FailedToDeserialize(c.base16_str().unwrap()))?
        .to_string())
}

/// Unwraps a hex-encoded String which is inside of a `Constant` acquired from a register of an `ErgoBox`.
pub fn unwrap_hex_encoded_string(c: &Constant) -> Result<String> {
    let byte_array: Result<Vec<u8>> = match Vec::<i8>::try_extract_from(c.clone()) {
        Ok(ba) => Ok(convert_to_unsigned_bytes(&ba)),
        _ => Err(EncodingError::FailedToUnwrap(c.base16_str().unwrap())),
    };
    Ok(base16::encode_lower(&byte_array?))
}

/// Acquire the `ErgoTree` of the P2S Base58 String.
pub fn deserialize_p2s_to_ergo_tree(p2s_address: P2SAddressString) -> Result<ErgoTree> {
    let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
    let address = encoder
        .parse_address_from_str(&p2s_address)
        .map_err(|_| EncodingError::FailedToDeserialize(p2s_address.clone()))?;
    ErgoTree::sigma_parse_bytes(&address.content_bytes())
        .map_err(|_| EncodingError::FailedToDeserialize(p2s_address.clone()))
}

/// Acquires the Base58 encoded P2S Address from an `ErgoTree`
pub fn serialize_p2s_from_ergo_tree(ergo_tree: ErgoTree) -> P2SAddressString {
    let address = Address::P2S(ergo_tree.sigma_serialize_bytes().unwrap());
    let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
    encoder.address_to_str(&address)
}

/// Attempts to acquire the Base58 encoded P2S or P2PK Address from an `ErgoTree`
pub fn serialize_address_from_ergo_tree(ergo_tree: ErgoTree) -> Result<ErgoAddressString> {
    if let Ok(address) = Address::recreate_from_ergo_tree(&ergo_tree) {
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        return Ok(encoder.address_to_str(&address));
    }
    Err(EncodingError::FailedToSerialize(
        "Failed to serialize ErgoTree".to_string(),
    ))
}

/// Deserialize ErgoTree inside of a `Constant` acquired from a register of an `ErgoBox` into a P2S Base58 String.
pub fn deserialize_ergo_tree_constant(c: &Constant) -> Result<P2SAddressString> {
    let byte_array: Result<Vec<u8>> = match Vec::<i8>::try_extract_from(c.clone()) {
        Ok(ba) => Ok(convert_to_unsigned_bytes(&ba)),
        _ => Err(EncodingError::FailedToUnwrap(c.base16_str().unwrap())),
    };

    let address = Address::P2S(byte_array?);
    let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);

    Ok(encoder.address_to_str(&address))
}

// Takes an Ergo address and transforms it into a mainnet address
// if it fails then it tries to recover by assumming it is a testnet address
pub fn parse_address(address_str: &ErgoAddressString) -> Result<Address> {
    let mainnet_encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
    let testnet_encoder = AddressEncoder::new(NetworkPrefix::Testnet);
    let address = mainnet_encoder.parse_address_from_str(address_str);
    return match address {
        Ok(addr) => Ok(addr),
        Err(error) => testnet_encoder
            .parse_address_from_str(address_str)
            .map_err(|_| EncodingError::FailedToSerialize(address_str.to_string())),
    };
}

/// Takes an Ergo address (either P2PK or P2S) as a Base58 String and returns
/// the `ErgoTree` if it is a valid address.
pub fn address_string_to_ergo_tree(address_str: &ErgoAddressString) -> Result<ErgoTree> {
    let address = parse_address(address_str)?;
    let ergo_tree = address
        .script()
        .map_err(|_| EncodingError::FailedToSerialize(address_str.to_string()))?;
    Ok(ergo_tree)
}

/// Builds a `TokenAmount` struct
pub fn build_token_amount(token_amount: u64) -> Result<TokenAmount> {
    Ok(TokenAmount::try_from(token_amount)?)
}

/// Builds a `TokenId` struct
pub fn build_token_id(token_id: &str) -> Result<TokenId> {
    Ok(TokenId::from(Digest32::try_from(
        Base16DecodedBytes::try_from(token_id.to_string())
            .map_err(|_| EncodingError::FailedToSerialize(token_id.to_string()))?,
    )?))
}

/// Builds a `Token` struct using the provided `token_id` and `token_amount`.
/// Useful when manually constructing transaction logic and requiring
/// building `Token`s when dealing with the id as a String and the amount
/// as a u64.
pub fn build_token(token_id: &str, token_amount: u64) -> Result<Token> {
    let token_amount = build_token_amount(token_amount)?;
    let token_id = build_token_id(token_id)?;
    Ok(Token {
        token_id: token_id,
        amount: token_amount,
    })
}

/// Convert Vec<i8> to Vec<u8>
fn convert_to_unsigned_bytes(bytes: &Vec<i8>) -> Vec<u8> {
    bytes.iter().map(|x| x.clone() as u8).collect()
}

/// Convert Vec<u8> to Vec<i8>
fn convert_to_signed_bytes(bytes: &Vec<u8>) -> Vec<i8> {
    bytes.iter().map(|x| x.clone() as i8).collect()
}

/// Decodes a hex-encoded string into bytes
fn decode_hex(s: &String) -> Result<Vec<u8>> {
    if let Ok(b) = base16::decode(s) {
        return Ok(b);
    } else {
        return Err(EncodingError::FailedToSerialize(s.clone()));
    }
}

/// Convert from Erg to nanoErg
pub fn erg_to_nano_erg(erg_amount: f64) -> u64 {
    (erg_amount * 1000000000 as f64) as u64
}

/// Convert from nanoErg to Erg
pub fn nano_erg_to_erg(nanoerg_amount: u64) -> f64 {
    (nanoerg_amount as f64) / (1000000000 as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erg_conv_is_valid() {
        assert_eq!((1 as f64), nano_erg_to_erg(1000000000));
        assert_eq!((1.23 as f64), nano_erg_to_erg(1230000000));

        assert_eq!(1000000000, erg_to_nano_erg(1 as f64));
        assert_eq!(erg_to_nano_erg(3.64), 3640000000);
        assert_eq!(erg_to_nano_erg(0.64), 640000000);
        assert_eq!(erg_to_nano_erg(0.0064), 6400000);
        assert_eq!(erg_to_nano_erg(0.000000064), 64);
        assert_eq!(erg_to_nano_erg(0.000000001), 1);
    }

    #[test]
    fn build_token_test() {
        let t = build_token(
            "0fb1eca4646950743bc5a8c341c16871a0ad9b4077e3b276bf93855d51a042d1",
            100000,
        )
        .unwrap();
    }
}
