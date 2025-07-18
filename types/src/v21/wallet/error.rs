// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::hex;
use bitcoin::psbt::PsbtParseError;

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `BumpFee` type into the model type.
#[derive(Debug)]
pub enum PsbtBumpFeeError {
    /// Conversion of the `psbt` field failed.
    Psbt(PsbtParseError),
    /// Conversion of the `original_fee` field failed.
    OriginalFee(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for PsbtBumpFeeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PsbtBumpFeeError as E;

        match *self {
            E::Psbt(ref e) => write_err!(f, "conversion of the `psbt` field failed"; e),
            E::OriginalFee(ref e) =>
                write_err!(f, "conversion of the `original_fee` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PsbtBumpFeeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use PsbtBumpFeeError as E;

        match *self {
            E::Psbt(ref e) => Some(e),
            E::OriginalFee(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}

/// Error when converting a `Send` type into the model type.
#[derive(Debug)]
pub enum SendError {
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `hex` field failed.
    Hex(encode::FromHexError),
    /// Conversion of the `psbt` field failed.
    Psbt(PsbtParseError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SendError as E;

        match *self {
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Psbt(ref e) => write_err!(f, "conversion of the `psbt` field failed"; e),
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SendError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use SendError as E;

        match *self {
            E::Txid(ref e) => Some(e),
            E::Hex(ref e) => Some(e),
            E::Psbt(ref e) => Some(e),
            E::Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for SendError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
