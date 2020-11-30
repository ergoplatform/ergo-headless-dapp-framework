pub mod box_spec;
pub mod box_traits;
pub mod encoding;
pub mod error;
pub mod output_builders;
pub mod specified_boxes;
pub mod tx_creation;

pub use box_spec::{BoxSpec, RegisterSpec, TokenSpec};
pub use box_traits::{ExplorerFindable, SpecifiedBox, WrappedBox};
pub use encoding::{erg_to_nano_erg, nano_erg_to_erg};
pub use ergo_headless_dapp_framework_derive::{SpecBox, WrapBox};
pub use ergo_lib::ast::constant::Constant;
pub use ergo_lib::chain::ergo_box::ErgoBox;
pub use ergo_lib::chain::transaction::unsigned::UnsignedTransaction;
pub use ergo_lib::ergo_tree::ErgoTree;
pub use ergo_lib::types::stype::SType;
pub use error::{HeadlessDappError, Result};
pub use output_builders::{ChangeBox, TokensChangeBox, TxFeeBox};
pub use specified_boxes::{ErgUsdOraclePoolBox, ErgsBox};
pub use tx_creation::{create_candidate, find_and_sum_other_tokens};

/// A Base58 encoded String of an Ergo address. Can be either P2PK or P2S.
pub type ErgoAddressString = String;
/// A Base58 encoded String of a Ergo P2PK address.
pub type P2PKAddressString = String;
/// A Base58 encoded String of a Ergo P2S address.
pub type P2SAddressString = String;
/// Transaction ID
pub type TxId = String;
/// The smallest unit of the Erg currency.
pub type NanoErg = u64;
/// A block height of the chain.
pub type BlockHeight = u64;
/// Duration in number of blocks.
pub type BlockDuration = u64;
/// A Base58 encoded String of a Token ID.
pub type TokenID = String;
/// Integer which is provided by the Ergo node to reference a given scan.
pub type ScanID = String;
