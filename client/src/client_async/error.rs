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

pub(crate) enum ParsedPreV29OrV29Plus<TPreV29, TV29Plus> {
    PreV29(TPreV29),
    V29Plus(TV29Plus),
}

pub(crate) enum ParsedPreV29OrV29ToV30OrV31Plus<TPreV29, TV29ToV30, TV31Plus> {
    PreV29(TPreV29),
    V29ToV30(TV29ToV30),
    V31Plus(TV31Plus),
}

pub(crate) enum ParsedPreV28OrV28OrV29Plus<TPreV28, T28, TV29Plus> {
    PreV28(TPreV28),
    V28(T28),
    V29Plus(TV29Plus),
}

/// Error returned when a response cannot be parsed as either pre-v29 or v29+.
#[derive(Debug)]
pub struct ParsePreV29OrV29PlusResponseError {
    /// Error returned when parsing as a pre-v29 response failed.
    pub pre_v29: serde_json::Error,
    /// Error returned when parsing as a v29+ response failed.
    pub v29_plus: serde_json::Error,
}

impl fmt::Display for ParsePreV29OrV29PlusResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "parsing the response as a pre-v29 version failed: {}; parsing the response as a v29+ version failed: {}",
            self.pre_v29, self.v29_plus
        )
    }
}

impl error::Error for ParsePreV29OrV29PlusResponseError {
    // Cannot expose both parse errors through a single `source()`; callers can inspect both
    // underlying errors via `pre_v29_error()` and `v29_plus_error()`.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
}

impl ParsePreV29OrV29PlusResponseError {
    /// Returns the pre-v29 parse error.
    pub fn pre_v29_error(&self) -> &serde_json::Error { &self.pre_v29 }

    /// Returns the v29+ parse error.
    pub fn v29_plus_error(&self) -> &serde_json::Error { &self.v29_plus }
}

/// Error returned when a response cannot be parsed as pre-v29, v29-v30, or v31+.
#[derive(Debug)]
pub struct ParsePreV29OrV29ToV30OrV31PlusResponseError {
    /// Error returned when parsing as a pre-v29 response failed.
    pub pre_v29: serde_json::Error,
    /// Error returned when parsing as a v29-v30 response failed.
    pub v29_to_v30: serde_json::Error,
    /// Error returned when parsing as a v31+ response failed.
    pub v31_plus: serde_json::Error,
}

impl fmt::Display for ParsePreV29OrV29ToV30OrV31PlusResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "parsing the response as a pre-v29 version failed: {}; parsing the response as a v29-v30 version failed: {}; parsing the response as a v31+ version failed: {}",
            self.pre_v29, self.v29_to_v30, self.v31_plus
        )
    }
}

impl error::Error for ParsePreV29OrV29ToV30OrV31PlusResponseError {
    // Cannot expose all parse errors through a single `source()`; callers can inspect them via
    // `pre_v29_error()`, `v29_to_v30_error()`, and `v31_plus_error()`.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
}

impl ParsePreV29OrV29ToV30OrV31PlusResponseError {
    /// Returns the pre-v29 parse error.
    pub fn pre_v29_error(&self) -> &serde_json::Error { &self.pre_v29 }

    /// Returns the v29-v30 parse error.
    pub fn v29_to_v30_error(&self) -> &serde_json::Error { &self.v29_to_v30 }

    /// Returns the v31+ parse error.
    pub fn v31_plus_error(&self) -> &serde_json::Error { &self.v31_plus }
}

macro_rules! impl_from_response_parse_error {
    ($error:ty, $variant:ident, $source:ty) => {
        impl From<$source> for $error {
            fn from(e: $source) -> Self { Self::$variant(e) }
        }
    };
}

/// Error returned when a response cannot be parsed as pre-v28, v28, or v29+.
#[derive(Debug)]
pub struct ParsePreV28OrV28OrV29PlusResponseError {
    /// Error returned when parsing as a pre-v28 response failed.
    pub pre_v28: serde_json::Error,
    /// Error returned when parsing as v28 failed.
    pub v28: serde_json::Error,
    /// Error returned when parsing as a v29+ response failed.
    pub v29_plus: serde_json::Error,
}

impl fmt::Display for ParsePreV28OrV28OrV29PlusResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "parsing the response as a pre-v28 version failed: {}; parsing the response as v28 failed: {}; parsing the response as a v29+ version failed: {}",
            self.pre_v28, self.v28, self.v29_plus
        )
    }
}

impl error::Error for ParsePreV28OrV28OrV29PlusResponseError {
    // Cannot expose all parse errors through a single `source()`; callers can inspect them via
    // `pre_v28_error()`, `v28_error()`, and `v29_plus_error()`.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { None }
}

impl ParsePreV28OrV28OrV29PlusResponseError {
    /// Returns the pre-v28 parse error.
    pub fn pre_v28_error(&self) -> &serde_json::Error { &self.pre_v28 }

    /// Returns the v28 parse error.
    pub fn v28_error(&self) -> &serde_json::Error { &self.v28 }

    /// Returns the v29+ parse error.
    pub fn v29_plus_error(&self) -> &serde_json::Error { &self.v29_plus }
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
    /// Deserializing the response into any supported schema failed.
    Response(ParsePreV29OrV29PlusResponseError),
    /// Conversion of a pre-v29 response into the model type failed.
    PreV29(types::v17::GetBlockHeaderVerboseError),
    /// Conversion of a v29+ response into the model type failed.
    V29Plus(types::v29::GetBlockHeaderVerboseError),
}

impl From<Error> for GetBlockHeaderVerboseError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl From<serde_json::Error> for GetBlockHeaderVerboseError {
    fn from(e: serde_json::Error) -> Self { Self::Json(Error::from(e)) }
}

impl_from_response_parse_error!(
    GetBlockHeaderVerboseError,
    Response,
    ParsePreV29OrV29PlusResponseError
);

impl fmt::Display for GetBlockHeaderVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblockheader verbose RPC failed: {}", e),
            Self::Response(ref e) => {
                write!(f, "getblockheader verbose response parsing failed: {}", e)
            }
            Self::PreV29(ref e) => {
                write!(f, "getblockheader verbose model conversion failed: {}", e)
            }
            Self::V29Plus(ref e) => {
                write!(f, "getblockheader verbose model conversion failed: {}", e)
            }
        }
    }
}

impl error::Error for GetBlockHeaderVerboseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::Response(ref e) => Some(e),
            Self::PreV29(ref e) => Some(e),
            Self::V29Plus(ref e) => Some(e),
        }
    }
}

/// Error returned by [`super::Client::get_block_verbose`].
#[derive(Debug)]
pub enum GetBlockVerboseError {
    Json(Error),
    /// Deserializing the response into any supported schema failed.
    Response(ParsePreV29OrV29ToV30OrV31PlusResponseError),
    /// Conversion of a pre-v29 response into the model type failed.
    PreV29(types::v17::GetBlockVerboseOneError),
    /// Conversion of a v29-v30 response into the model type failed.
    V29ToV30(types::v29::GetBlockVerboseOneError),
    /// Conversion of a v31+ response into the model type failed.
    V31Plus(types::v31::GetBlockVerboseOneError),
}

impl From<Error> for GetBlockVerboseError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl From<serde_json::Error> for GetBlockVerboseError {
    fn from(e: serde_json::Error) -> Self { Self::Json(Error::from(e)) }
}

impl_from_response_parse_error!(
    GetBlockVerboseError,
    Response,
    ParsePreV29OrV29ToV30OrV31PlusResponseError
);

impl fmt::Display for GetBlockVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblock verbose RPC failed: {}", e),
            Self::Response(ref e) => write!(f, "getblock verbose response parsing failed: {}", e),
            Self::PreV29(ref e) => write!(f, "getblock verbose model conversion failed: {}", e),
            Self::V29ToV30(ref e) => {
                write!(f, "getblock verbose model conversion failed: {}", e)
            }
            Self::V31Plus(ref e) => write!(f, "getblock verbose model conversion failed: {}", e),
        }
    }
}

impl error::Error for GetBlockVerboseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::Response(ref e) => Some(e),
            Self::PreV29(ref e) => Some(e),
            Self::V29ToV30(ref e) => Some(e),
            Self::V31Plus(ref e) => Some(e),
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
    /// Deserializing the response into any supported schema failed.
    Response(ParsePreV28OrV28OrV29PlusResponseError),
    /// Conversion of a pre-v28 response into the model type failed.
    PreV28(types::v23::GetBlockchainInfoError),
    /// Conversion of a v28 response into the model type failed.
    V28(types::v28::GetBlockchainInfoError),
    /// Conversion of a v29+ response into the model type failed.
    V29Plus(types::v29::GetBlockchainInfoError),
}

impl From<Error> for GetBlockchainInfoError {
    fn from(e: Error) -> Self { Self::Json(e) }
}

impl From<serde_json::Error> for GetBlockchainInfoError {
    fn from(e: serde_json::Error) -> Self { Self::Json(Error::from(e)) }
}

impl_from_response_parse_error!(
    GetBlockchainInfoError,
    Response,
    ParsePreV28OrV28OrV29PlusResponseError
);

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Json(ref e) => write!(f, "getblockchaininfo RPC failed: {}", e),
            Self::Response(ref e) => {
                write!(f, "getblockchaininfo response parsing failed: {}", e)
            }
            Self::PreV28(ref e) => {
                write!(f, "getblockchaininfo model conversion failed: {}", e)
            }
            Self::V28(ref e) => {
                write!(f, "getblockchaininfo model conversion failed: {}", e)
            }
            Self::V29Plus(ref e) => {
                write!(f, "getblockchaininfo model conversion failed: {}", e)
            }
        }
    }
}

impl error::Error for GetBlockchainInfoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Json(ref e) => Some(e),
            Self::Response(ref e) => Some(e),
            Self::PreV28(ref e) => Some(e),
            Self::V28(ref e) => Some(e),
            Self::V29Plus(ref e) => Some(e),
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

pub(crate) fn decode_pre_v29_or_v29_plus<TPreV29, TV29Plus>(
    raw: &serde_json::value::RawValue,
) -> std::result::Result<ParsedPreV29OrV29Plus<TPreV29, TV29Plus>, ParsePreV29OrV29PlusResponseError>
where
    TPreV29: for<'a> serde::de::Deserialize<'a>,
    TV29Plus: for<'a> serde::de::Deserialize<'a>,
{
    match serde_json::from_str::<TV29Plus>(raw.get()) {
        Ok(json) => Ok(ParsedPreV29OrV29Plus::V29Plus(json)),
        Err(v29_plus) => match serde_json::from_str::<TPreV29>(raw.get()) {
            Ok(json) => Ok(ParsedPreV29OrV29Plus::PreV29(json)),
            Err(pre_v29) => Err(ParsePreV29OrV29PlusResponseError { pre_v29, v29_plus }),
        },
    }
}

pub(crate) fn decode_pre_v29_or_v29_to_v30_or_v31_plus<TPreV29, TV29ToV30, TV31Plus>(
    raw: &serde_json::value::RawValue,
) -> std::result::Result<
    ParsedPreV29OrV29ToV30OrV31Plus<TPreV29, TV29ToV30, TV31Plus>,
    ParsePreV29OrV29ToV30OrV31PlusResponseError,
>
where
    TPreV29: for<'a> serde::de::Deserialize<'a>,
    TV29ToV30: for<'a> serde::de::Deserialize<'a>,
    TV31Plus: for<'a> serde::de::Deserialize<'a>,
{
    match serde_json::from_str::<TV31Plus>(raw.get()) {
        Ok(json) => Ok(ParsedPreV29OrV29ToV30OrV31Plus::V31Plus(json)),
        Err(v31_plus) => match serde_json::from_str::<TV29ToV30>(raw.get()) {
            Ok(json) => Ok(ParsedPreV29OrV29ToV30OrV31Plus::V29ToV30(json)),
            Err(v29_to_v30) => match serde_json::from_str::<TPreV29>(raw.get()) {
                Ok(json) => Ok(ParsedPreV29OrV29ToV30OrV31Plus::PreV29(json)),
                Err(pre_v29) => Err(ParsePreV29OrV29ToV30OrV31PlusResponseError {
                    pre_v29,
                    v29_to_v30,
                    v31_plus,
                }),
            },
        },
    }
}

pub(crate) fn decode_pre_v28_or_v28_or_v29_plus<TPreV28, T28, TV29Plus>(
    raw: &serde_json::value::RawValue,
) -> std::result::Result<
    ParsedPreV28OrV28OrV29Plus<TPreV28, T28, TV29Plus>,
    ParsePreV28OrV28OrV29PlusResponseError,
>
where
    TPreV28: for<'a> serde::de::Deserialize<'a>,
    T28: for<'a> serde::de::Deserialize<'a>,
    TV29Plus: for<'a> serde::de::Deserialize<'a>,
{
    match serde_json::from_str::<TV29Plus>(raw.get()) {
        Ok(json) => Ok(ParsedPreV28OrV28OrV29Plus::V29Plus(json)),
        Err(v29_plus) => match serde_json::from_str::<T28>(raw.get()) {
            Ok(json) => Ok(ParsedPreV28OrV28OrV29Plus::V28(json)),
            Err(v28) => match serde_json::from_str::<TPreV28>(raw.get()) {
                Ok(json) => Ok(ParsedPreV28OrV28OrV29Plus::PreV28(json)),
                Err(pre_v28) =>
                    Err(ParsePreV28OrV28OrV29PlusResponseError { pre_v28, v28, v29_plus }),
            },
        },
    }
}
