use crate::box_spec::BoxSpec;
use crate::encoding::serialize_p2s_from_ergo_tree;
use crate::error::Result;
use crate::{NanoErg, P2SAddressString};
use ergo_lib::chain::transaction::DataInput;
use ergo_lib::chain::transaction::UnsignedInput;
use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;
use ergo_lib::ergotree_ir::chain::token::Token;
use ergo_lib::ergotree_ir::mir::constant::Constant;

/// A trait which represents an `ErgoBox` wrapped in an overarching struct.
pub trait WrappedBox {
    fn get_box(&self) -> ErgoBox;
    /// Converts the `WrappedBox` into a `DataInput`
    fn as_data_input(&self) -> DataInput {
        self.get_box().box_id().into()
    }
    /// Converts the `WrappedBox` into an `UnsignedInput`
    fn as_unsigned_input(&self) -> UnsignedInput {
        self.get_box().into()
    }
    /// Returns the Box ID of the wrapped `ErgoBox` as a base16 String
    fn box_id(&self) -> String {
        self.get_box().box_id().into()
    }
    /// Returns the amount of nanoErgs held in the wrapped `ErgoBox` as u64
    fn nano_ergs(&self) -> NanoErg {
        self.get_box().value.as_u64().clone()
    }
    // Returns the P2S Address of wrapped `ErgoBox` as a String
    fn p2s_address(&self) -> P2SAddressString {
        serialize_p2s_from_ergo_tree(self.get_box().ergo_tree)
    }
    /// Returns the registers of the wrapped `ErgoBox` as an ordered Vector
    /// of `Constant`s. First element is R4, second element is R5, etc.
    fn registers(&self) -> Vec<Constant> {
        self.get_box()
            .additional_registers
            .get_ordered_values()
            .clone()
    }
    /// Returns the `Token`s inside of the wrapped `ErgoBox`
    fn tokens(&self) -> Vec<Token> {
        self.get_box().tokens
    }
    /// Returns the creation height of the wrapped `ErgoBox`
    fn creation_height(&self) -> u64 {
        self.get_box().creation_height.clone() as u64
    }
}

pub trait SpecifiedBox: WrappedBox {
    // Associated fn which returns the `BoxSpec` for said `SpecifiedBox`
    fn box_spec() -> BoxSpec;

    // Acquire UTXO-set scan JSON from the `BoxSpec`
    fn get_utxo_scan_json_string() -> String {
        Self::box_spec().utxo_scan_json()
    }

    /// Verify that a provided `ErgoBox` matches the `BoxSpec`
    /// tied to your `SpecifiedBox`
    fn verify_box(ergo_box: &ErgoBox) -> Result<()> {
        Self::box_spec().verify_box(ergo_box)
    }

    /// Generates a URL for the Ergo Explorer Backend API
    /// to find boxes which may match your `BoxSpec`. This method uses
    /// the `explorer_api_url` you provide as input which
    /// must be formatted as such:
    /// `https://api.ergoplatform.com/api`
    /// This method is intended to be used in tandem with
    /// `process_explorer_response()`
    fn explorer_endpoint(explorer_api_url: &str) -> Result<String> {
        Self::box_spec().explorer_endpoint(explorer_api_url)
    }
}

/// A trait which is implemented via deriving the procedural macro `SpecBox`.
/// This trait wraps the below methods from `BoxSpec` but instead returns
/// the `Self` struct that implements `SpecifiedBox` improving the dev
/// experience.
/// A separate trait + using a proc macro was required due to a few
/// intricacies of Rust's trait/type system
pub trait ExplorerFindable: SpecifiedBox {
    /// Using the response JSON (as a String) from the Ergo Explorer API
    /// endpoint generated by the `explorer_endpoint()` method,
    /// filter all returned `ErgoBox`es against the default `BoxSpec`
    /// of the `SpecifiedBox` using the `verify_box()` method and creating
    /// new instances of your `SpecifiedBox` struct.
    fn process_explorer_response(explorer_response_body: &str) -> Result<Vec<Self>>
    where
        Self: Sized;

    /// Using the response JSON (as a String) from the Ergo Explorer API
    /// endpoint generated by the `explorer_endpoint()` method,
    /// filter all returned `ErgoBox`es against a custom `BoxSpec`
    /// using the `verify_box()` method and creating new instances
    /// of your `SpecifiedBox` struct.
    fn process_explorer_response_custom(
        explorer_response_body: &str,
        box_spec: BoxSpec,
    ) -> Result<Vec<Self>>
    where
        Self: Sized;
}
