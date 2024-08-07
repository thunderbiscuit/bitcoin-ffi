use bitcoin::Amount as BitcoinAmount;
use bitcoin::ScriptBuf as BitcoinScriptBuf;

use error::ParseAmountError;

pub mod error;
pub use bitcoin::amount::ParseAmountError as BitcoinParseAmountError;
pub use bitcoin::Network;

macro_rules! impl_from_core_type {
    ($inner:ident, $outer:ident) => {
        impl From<$inner> for $outer {
            fn from(inner: $inner) -> Self {
                $outer(inner)
            }
        }
    };
}

macro_rules! impl_from_outer_type {
    ($inner:ident, $outer:ident) => {
        impl From<$outer> for $inner {
            fn from(value: $outer) -> Self {
                value.0
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Script(pub BitcoinScriptBuf);

impl Script {
    pub fn new(raw_output_script: Vec<u8>) -> Self {
        let script: BitcoinScriptBuf = raw_output_script.into();
        Script(script)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl_from_core_type!(BitcoinScriptBuf, Script);

pub struct Amount(pub BitcoinAmount);

impl Amount {
    pub fn from_sat(sat: u64) -> Self {
        Amount(BitcoinAmount::from_sat(sat))
    }

    pub fn from_btc(btc: f64) -> Result<Self, ParseAmountError> {
        let bdk_amount = BitcoinAmount::from_btc(btc).map_err(ParseAmountError::from)?;
        Ok(Amount(bdk_amount))
    }

    pub fn to_sat(&self) -> u64 {
        self.0.to_sat()
    }

    pub fn to_btc(&self) -> f64 {
        self.0.to_btc()
    }
}

impl_from_core_type!(BitcoinAmount, Amount);
impl_from_outer_type!(BitcoinAmount, Amount);

uniffi::include_scaffolding!("bitcoin");
