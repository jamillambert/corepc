// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::hex;

/// Error when converting [`ScanBlocksStart`] to [`model::ScanBlocksStart`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScanBlocksStartError {
    /// Conversion of block hash failed.
    BlockHash {
        /// The field that we failed to convert.
        field: &'static str,
        /// The hex string we attempted to parse.
        hex: String,
        /// The error returned by the hex parsing code.
        error: hex::HexToArrayError,
    },
}

impl fmt::Display for ScanBlocksStartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ScanBlocksStartError::*;

        match *self {
            BlockHash { field, ref hex, ref error } => {
                write!(f, "failed to parse block hash for field {}: {} ({})", field, hex, error)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ScanBlocksStartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ScanBlocksStartError::*;

        match *self {
            BlockHash { ref error, .. } => Some(error),
        }
    }
}
