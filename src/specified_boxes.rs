use crate::box_spec::BoxSpec;
use crate::box_traits::{ExplorerFindable, SpecifiedBox, WrappedBox};
use crate::encoding::unwrap_long;
use crate::error::{HeadlessDappError, Result};
use crate::SType::SLong;
/// This file holds a number of default general "Specified Boxes".
/// These are wrapper structs for `ErgoBox`es which meet a given
/// specification and provide you with a simple interface
/// for implementing Actions of your protocols.
use crate::WASMBox;
use crate::{RegisterSpec, TokenSpec};
use ergo_headless_dapp_framework_derive::{SpecBox, WrapBox};
use ergo_lib::chain::ergo_box::ErgoBox;
use ergo_lib_wasm::box_coll::ErgoBoxes;
use ergo_lib_wasm::ergo_box::ErgoBox as WErgoBox;
use wasm_bindgen::prelude::*;

/// A specified box which is intended to be spent for the Ergs inside.
/// The spec simply requires the box to simply have at least `1000000`
/// nanoErgs inside.
#[wasm_bindgen]
#[derive(Clone, Debug, WrapBox, SpecBox, WASMBox, Eq, PartialEq)]
pub struct ErgsBox {
    ergo_box: ErgoBox,
}
/// SpecifiedBox impl
impl SpecifiedBox for ErgsBox {
    /// A simple `BoxSpec` that just checks the value of nanoErgs is
    /// above `1000000`
    fn box_spec() -> BoxSpec {
        BoxSpec::new(None, Some(1000000..u64::MAX), vec![], vec![])
    }
}
/// Rust ErgsBox Methods
impl ErgsBox {
    /// Converts from the WASM wrapper `ErgoBoxes` into a vector of
    /// `ErgsBox`es.
    pub fn convert_from_ergo_boxes(ergo_boxes: &ErgoBoxes) -> Result<Vec<ErgsBox>> {
        // Mutable list of `ErgsBox`es
        let mut ergs_boxes: Vec<ErgsBox> = vec![];
        // Unwrapped list of `ErgoBox`es
        let unwrapped_boxes: Vec<ErgoBox> = ergo_boxes.clone().into();
        // Converting all unwrapped `ErgoBox`es into `ErgsBox`es
        for b in unwrapped_boxes {
            let ergs_box = ErgsBox::new(&b)?;
            ergs_boxes.push(ergs_box);
        }
        Ok(ergs_boxes)
    }

    /// Sums the nanoErg value of a list of `ErgsBox`es
    pub fn sum_ergs_boxes_value(boxes: &Vec<ErgsBox>) -> u64 {
        boxes
            .into_iter()
            .fold(0, |acc, pb| pb.get_box().value.as_u64().clone() + acc)
    }
}

/// A specified box which is an Oracle Pool box that stores a `Long` integer
/// datapoint inside of R4 that represents how many nanoErgs can be bought
/// for 1 USD.
#[wasm_bindgen]
#[derive(Clone, Debug, WrapBox, SpecBox, WASMBox)]
pub struct ErgUsdOraclePoolBox {
    ergo_box: ErgoBox,
}
/// SpecifiedBox impl
impl SpecifiedBox for ErgUsdOraclePoolBox {
    /// A box spec for an Oracle Pool Box with the correct NFT + a Long value
    /// in R4
    fn box_spec() -> BoxSpec {
        let registers = vec![RegisterSpec::new(Some(SLong), None)];
        let tokens = vec![Some(TokenSpec::new(
            1..2,
            "008a94c8c76bbaa1f0a346697d1794eb31d94b37e5533af9cc0b6932bf159339",
        ))];
        BoxSpec::new(None, None, registers, tokens)
    }
}
/// WASM-compatible ErgUsdOraclePoolBox Methods
#[wasm_bindgen]
impl ErgUsdOraclePoolBox {
    #[wasm_bindgen]
    /// Extracts the Long datapoint out of register R4.
    pub fn datapoint(&self) -> u64 {
        return unwrap_long(&self.registers()[0]).unwrap() as u64;
    }

    #[wasm_bindgen]
    /// Extracts the Long datapoint out of register R4.
    pub fn datapoint_in_cents(&self) -> u64 {
        return (self.datapoint() / 100) as u64;
    }
}

/// A specified box which is an Oracle Pool box that stores a `Long` integer
/// datapoint inside of R4 that represents how many lovelaces can be bought
/// for 1 USD.
#[wasm_bindgen]
#[derive(Clone, Debug, WrapBox, SpecBox, WASMBox)]
pub struct AdaUsdOraclePoolBox {
    ergo_box: ErgoBox,
}
/// SpecifiedBox impl
impl SpecifiedBox for AdaUsdOraclePoolBox {
    /// A box spec for an Oracle Pool Box with the correct NFT + a Long value
    /// in R4
    fn box_spec() -> BoxSpec {
        let registers = vec![RegisterSpec::new(Some(SLong), None)];
        let tokens = vec![Some(TokenSpec::new(
            1..2,
            "19475d9a78377ff0f36e9826cec439727bea522f6ffa3bda32e20d2f8b3103ac",
        ))];
        BoxSpec::new(None, None, registers, tokens)
    }
}
/// WASM-compatible AdaUsdOraclePoolBox Methods
#[wasm_bindgen]
impl AdaUsdOraclePoolBox {
    #[wasm_bindgen]
    /// Extracts the Long datapoint out of register R4.
    pub fn datapoint(&self) -> u64 {
        return unwrap_long(&self.registers()[0]).unwrap() as u64;
    }

    #[wasm_bindgen]
    /// Extracts the Long datapoint out of register R4.
    pub fn datapoint_in_cents(&self) -> u64 {
        return (self.datapoint() / 100) as u64;
    }
}
