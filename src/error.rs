use crate::encoding::EncodingError;
use crate::{NanoErg, P2PKAddressString, P2SAddressString};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, HeadlessDappError>;

#[derive(Error, Debug)]
pub enum HeadlessDappError {
    #[error("The address of the box being verified does not match the `BoxSpec`.")]
    InvalidSpecAddress,
    #[error(
        "The number of Ergs held within the box is outside of the valid range for the `BoxSpec`."
    )]
    InvalidSpecErgsValue,
    #[error("One of the tokens failed to match the `BoxSpec`.")]
    FailedTokenSpec,
    #[error("One of the registers failed to match the `BoxSpec`.")]
    FailedRegisterSpec,
    #[error("The encoded predicate on the BoxSpec failed.")]
    FailedSpecPredicate,
    #[error("The address provided is invalid: {0}")]
    InvalidAddress(String),
    #[error("The Box value {0} is invalid.")]
    InvalidBoxValue(NanoErg),
    #[error("Invalid P2S Address: {0}")]
    InvalidP2SAddress(P2SAddressString),
    #[error("Invalid P2PK Address: {0}")]
    InvalidP2PKAddress(P2PKAddressString),
    #[error("The values attempted to be encoded within registers failed.")]
    InvalidRegisterValues(),
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    EncodeError(#[from] EncodingError<String>),
}
