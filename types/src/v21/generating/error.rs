// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::hex;

use crate::error::write_err;

/// Error when converting a `GenerateBlock` type into the model type.
#[derive(Debug)]
pub enum GenerateBlockError {
    /// Conversion of the `hash` field failed.
    Hash(hex::HexToArrayError),
}

impl fmt::Display for GenerateBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GenerateBlockError::*;

        match *self {
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GenerateBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GenerateBlockError::*;

        match *self {
            Hash(ref e) => Some(e),
        }
    }
}
