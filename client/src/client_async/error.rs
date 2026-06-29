// SPDX-License-Identifier: CC0-1.0

use std::{error, fmt, io};

use bitcoin::consensus::encode;
use bitcoin::hex;

use crate::types;

macro_rules! impl_from_json_error {
    ($($error:ty),* $(,)?) => {
        $(
            impl From<serde_json::Error> for $error {
                fn from(e: serde_json::Error) -> Self { Self::Json(Error::from(e)) }
            }
        )*
    }
}

/// The error type for errors produced in this library.
#[derive(Debug)]
pub enum Error {
    JsonRpc(jsonrpc::error::Error),
    HexToArray(hex::HexToArrayError),
    HexToBytes(hex::HexToBytesError),
    Json(serde_json::error::Error),
    BitcoinSerialization(encode::FromHexError),
    Io(io::Error),
    InvalidCookieFile,
    /// The JSON result had an unexpected structure.
    UnexpectedStructure,
    /// The daemon returned an error string.
    Returned(String),
    /// The server version did not match what was expected.
    ServerVersion(UnexpectedServerVersionError),
    /// Missing user/password.
    MissingUserPassword,
}

impl From<jsonrpc::error::Error> for Error {
    fn from(e: jsonrpc::error::Error) -> Error { Error::JsonRpc(e) }
}

impl From<hex::HexToArrayError> for Error {
    fn from(e: hex::HexToArrayError) -> Self { Self::HexToArray(e) }
}

impl From<hex::HexToBytesError> for Error {
    fn from(e: hex::HexToBytesError) -> Self { Self::HexToBytes(e) }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error { Error::Json(e) }
}

impl From<encode::FromHexError> for Error {
    fn from(e: encode::FromHexError) -> Error { Error::BitcoinSerialization(e) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error { Error::Io(e) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            JsonRpc(ref e) => write!(f, "JSON-RPC error: {}", e),
            HexToArray(ref e) => write!(f, "hex to array decode error: {}", e),
            HexToBytes(ref e) => write!(f, "hex to bytes decode error: {}", e),
            Json(ref e) => write!(f, "JSON error: {}", e),
            BitcoinSerialization(ref e) => write!(f, "Bitcoin serialization error: {}", e),
            Io(ref e) => write!(f, "I/O error: {}", e),
            InvalidCookieFile => write!(f, "invalid cookie file"),
            UnexpectedStructure => write!(f, "the JSON result had an unexpected structure"),
            Returned(ref s) => write!(f, "the daemon returned an error string: {}", s),
            ServerVersion(ref e) => write!(f, "server version: {}", e),
            MissingUserPassword => write!(f, "missing user and/or password"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;

        match *self {
            JsonRpc(ref e) => Some(e),
            HexToArray(ref e) => Some(e),
            HexToBytes(ref e) => Some(e),
            Json(ref e) => Some(e),
            BitcoinSerialization(ref e) => Some(e),
            Io(ref e) => Some(e),
            ServerVersion(ref e) => Some(e),
            InvalidCookieFile | UnexpectedStructure | Returned(_) | MissingUserPassword => None,
        }
    }
}

/// Error returned when RPC client expects a different version than bitcoind reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnexpectedServerVersionError {
    /// Version from server.
    pub got: usize,
    /// Expected server version.
    pub expected: Vec<usize>,
}

impl fmt::Display for UnexpectedServerVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut expected = String::new();
        for version in &self.expected {
            let v = format!(" {} ", version);
            expected.push_str(&v);
        }
        write!(f, "unexpected bitcoind version, got: {} expected one of: {}", self.got, expected)
    }
}

impl error::Error for UnexpectedServerVersionError {}

impl From<UnexpectedServerVersionError> for Error {
    fn from(e: UnexpectedServerVersionError) -> Self { Self::ServerVersion(e) }
}

pub(crate) enum ParsedVersion<T25, T29> {
    V25(T25),
    V29(T29),
}

/// Error returned by [`super::Client::get_block`].
#[derive(Debug)]
pub enum GetBlockError {
    Json(Error),
    IntoModel(encode::FromHexError),
}

impl_from_json_error!(GetBlockError);

impl From<Error> for GetBlockError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblock RPC failed: {}", e),
            Self::IntoModel(ref e) => write!(f, "getblock model conversion failed: {}", e),
        }
    }
}

impl error::Error for GetBlockError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::IntoModel(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_block_count`].
#[derive(Debug)]
pub enum GetBlockCountError {
    Json(Error),
}

impl_from_json_error!(GetBlockCountError);

impl From<Error> for GetBlockCountError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBlockCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblockcount RPC failed: {}", e),
        }
    }
}

impl error::Error for GetBlockCountError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_block_hash`].
#[derive(Debug)]
pub enum GetBlockHashError {
    Json(Error),
    IntoModel(hex::HexToArrayError),
}

impl_from_json_error!(GetBlockHashError);

impl From<Error> for GetBlockHashError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBlockHashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblockhash RPC failed: {}", e),
            Self::IntoModel(ref e) => write!(f, "getblockhash model conversion failed: {}", e),
        }
    }
}

impl error::Error for GetBlockHashError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::IntoModel(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_best_block_hash`].
#[derive(Debug)]
pub enum GetBestBlockHashError {
    Json(Error),
    IntoModel(hex::HexToArrayError),
}

impl_from_json_error!(GetBestBlockHashError);

impl From<Error> for GetBestBlockHashError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBestBlockHashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getbestblockhash RPC failed: {}", e),
            Self::IntoModel(ref e) => {
                write!(f, "getbestblockhash model conversion failed: {}", e)
            }
        }
    }
}

impl error::Error for GetBestBlockHashError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::IntoModel(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_block_header`].
#[derive(Debug)]
pub enum GetBlockHeaderError {
    Json(Error),
    IntoModel(types::v25::GetBlockHeaderError),
}

impl_from_json_error!(GetBlockHeaderError);

impl From<Error> for GetBlockHeaderError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBlockHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblockheader RPC failed: {}", e),
            Self::IntoModel(ref e) => write!(f, "getblockheader model conversion failed: {}", e),
        }
    }
}

impl error::Error for GetBlockHeaderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::IntoModel(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_block_header_verbose`].
#[derive(Debug)]
pub enum GetBlockHeaderVerboseError {
    Json(Error),
    V25(types::v25::GetBlockHeaderVerboseError),
    V29(types::v29::GetBlockHeaderVerboseError),
}

impl_from_json_error!(GetBlockHeaderVerboseError);

impl From<Error> for GetBlockHeaderVerboseError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBlockHeaderVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblockheader verbose RPC failed: {}", e),
            Self::V25(ref e) => {
                write!(f, "getblockheader verbose model conversion failed: {}", e)
            }
            Self::V29(ref e) => {
                write!(f, "getblockheader verbose model conversion failed: {}", e)
            }
        }
    }
}

impl error::Error for GetBlockHeaderVerboseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::V25(ref e) => Some(e),
            Self::V29(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_block_verbose`].
#[derive(Debug)]
pub enum GetBlockVerboseError {
    Json(Error),
    V25(types::v25::GetBlockVerboseOneError),
    V29(types::v29::GetBlockVerboseOneError),
}

impl_from_json_error!(GetBlockVerboseError);

impl From<Error> for GetBlockVerboseError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBlockVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblock verbose RPC failed: {}", e),
            Self::V25(ref e) => write!(f, "getblock verbose model conversion failed: {}", e),
            Self::V29(ref e) => write!(f, "getblock verbose model conversion failed: {}", e),
        }
    }
}

impl error::Error for GetBlockVerboseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::V25(ref e) => Some(e),
            Self::V29(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_block_filter`].
#[derive(Debug)]
pub enum GetBlockFilterError {
    Json(Error),
    IntoModel(types::v19::GetBlockFilterError),
}

impl_from_json_error!(GetBlockFilterError);

impl From<Error> for GetBlockFilterError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBlockFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblockfilter RPC failed: {}", e),
            Self::IntoModel(ref e) => write!(f, "getblockfilter model conversion failed: {}", e),
        }
    }
}

impl error::Error for GetBlockFilterError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::IntoModel(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_blockchain_info`].
#[derive(Debug)]
pub enum GetBlockchainInfoError {
    Json(Error),
    V25(types::v25::GetBlockchainInfoError),
    V28(types::v28::GetBlockchainInfoError),
    V29(types::v29::GetBlockchainInfoError),
}

impl_from_json_error!(GetBlockchainInfoError);

impl From<Error> for GetBlockchainInfoError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblockchaininfo RPC failed: {}", e),
            Self::V25(ref e) => {
                write!(f, "getblockchaininfo model conversion failed: {}", e)
            }
            Self::V28(ref e) => {
                write!(f, "getblockchaininfo model conversion failed: {}", e)
            }
            Self::V29(ref e) => {
                write!(f, "getblockchaininfo model conversion failed: {}", e)
            }
        }
    }
}

impl error::Error for GetBlockchainInfoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::V25(ref e) => Some(e),
            Self::V28(ref e) => Some(e),
            Self::V29(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_raw_mempool`].
#[derive(Debug)]
pub enum GetRawMempoolError {
    Json(Error),
    IntoModel(hex::HexToArrayError),
}

impl_from_json_error!(GetRawMempoolError);

impl From<Error> for GetRawMempoolError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetRawMempoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getrawmempool RPC failed: {}", e),
            Self::IntoModel(ref e) => write!(f, "getrawmempool model conversion failed: {}", e),
        }
    }
}

impl error::Error for GetRawMempoolError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::IntoModel(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_raw_transaction`].
#[derive(Debug)]
pub enum GetRawTransactionError {
    Json(Error),
    IntoModel(encode::FromHexError),
}

impl_from_json_error!(GetRawTransactionError);

impl From<Error> for GetRawTransactionError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetRawTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getrawtransaction RPC failed: {}", e),
            Self::IntoModel(ref e) => {
                write!(f, "getrawtransaction model conversion failed: {}", e)
            }
        }
    }
}

impl error::Error for GetRawTransactionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::IntoModel(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_tx_out`].
#[derive(Debug)]
pub enum GetTxOutError {
    Json(Error),
    IntoModel(types::v25::GetTxOutError),
}

impl_from_json_error!(GetTxOutError);

impl From<Error> for GetTxOutError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for GetTxOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "gettxout RPC failed: {}", e),
            Self::IntoModel(ref e) => write!(f, "gettxout model conversion failed: {}", e),
        }
    }
}

impl error::Error for GetTxOutError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::IntoModel(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::server_version`].
#[derive(Debug)]
pub enum ServerVersionError {
    Json(Error),
}

impl_from_json_error!(ServerVersionError);

impl From<Error> for ServerVersionError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl fmt::Display for ServerVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "server_version RPC failed: {}", e),
        }
    }
}

impl error::Error for ServerVersionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::check_expected_server_version`].
#[derive(Debug)]
pub enum CheckServerVersionError {
    Json(ServerVersionError),
    Unexpected(UnexpectedServerVersionError),
}

impl fmt::Display for CheckServerVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "server version RPC failed: {}", e),
            Self::Unexpected(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for CheckServerVersionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::Unexpected(ref e) => Some(e),
        }
    }
}

impl From<ServerVersionError> for CheckServerVersionError {
    fn from(e: ServerVersionError) -> Self { Self::Json(e) }
}

impl From<UnexpectedServerVersionError> for CheckServerVersionError {
    fn from(e: UnexpectedServerVersionError) -> Self { Self::Unexpected(e) }
}

pub(crate) fn decode_v25_or_v29<T25, T29>(
    raw: &serde_json::value::RawValue,
) -> std::result::Result<ParsedVersion<T25, T29>, serde_json::Error>
where
    T25: for<'a> serde::de::Deserialize<'a>,
    T29: for<'a> serde::de::Deserialize<'a>,
{
    match serde_json::from_str::<T29>(raw.get()) {
        Ok(json) => Ok(ParsedVersion::V29(json)),
        Err(_) => serde_json::from_str::<T25>(raw.get()).map(ParsedVersion::V25),
    }
}
