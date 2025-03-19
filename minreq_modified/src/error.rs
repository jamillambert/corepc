extern crate alloc;

use std::{error, io, str};

use alloc::fmt;

/// Represents an error while sending, receiving, or parsing an HTTP response.
#[derive(Debug)]
// TODO: Make non-exhaustive for 3.0?
pub enum Error {
    #[cfg(feature = "json-using-serde")]
    /// Ran into a Serde error.
    SerdeJsonError(serde_json::Error),
    /// The response body contains invalid UTF-8, so the `as_str()`
    /// conversion failed.
    InvalidUtf8InBody(str::Utf8Error),

    /// Ran into an IO problem while loading the response.
    IoError(io::Error),
    /// Couldn't parse the incoming chunk's length while receiving a
    /// response with the header `Transfer-Encoding: chunked`.
    MalformedChunkLength,
    /// The chunk did not end after reading the previously read amount
    /// of bytes.
    MalformedChunkEnd,
    /// Couldn't parse the `Content-Length` header's value as an
    /// `usize`.
    MalformedContentLength,
    /// The response contains headers whose total size surpasses
    /// [Request::with_max_headers_size](crate::request::Request::with_max_headers_size).
    HeadersOverflow,
    /// The response's status line length surpasses
    /// [Request::with_max_status_line_size](crate::request::Request::with_max_status_line_length).
    StatusLineOverflow,
    /// [ToSocketAddrs](std::net::ToSocketAddrs) did not resolve to an
    /// address.
    AddressNotFound,
    /// The response was a redirection, but the `Location` header is
    /// missing.
    RedirectLocationMissing,
    /// The response redirections caused an infinite redirection loop.
    InfiniteRedirectionLoop,
    /// Followed
    /// [`max_redirections`](struct.Request.html#method.with_max_redirections)
    /// redirections, won't follow any more.
    TooManyRedirections,
    /// The response contained invalid UTF-8 where it should be valid
    /// (eg. headers), so the response cannot interpreted correctly.
    InvalidUtf8InResponse,
    /// Tried to send a secure request (ie. the url started with
    /// `https://`), but the crate's `https` feature was not enabled,
    /// and as such, a connection cannot be made.
    HttpsFeatureNotEnabled,
    /// The provided url contained a domain that has non-ASCII
    /// characters.
    NonASCIIurl,
    /// This is a special error case, one that should never be
    /// returned! Think of this as a cleaner alternative to calling
    /// `unreachable!()` inside the library. If you come across this,
    /// please open an issue, and include the string inside this
    /// error, as it can be used to locate the problem.
    Other(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            #[cfg(feature = "json-using-serde")]
            SerdeJsonError(err) => write!(f, "{}", err),
            IoError(err) => write!(f, "{}", err),
            InvalidUtf8InBody(err) => write!(f, "{}", err),

            MalformedChunkLength => write!(f, "non-usize chunk length with transfer-encoding: chunked"),
            MalformedChunkEnd => write!(f, "chunk did not end after reading the expected amount of bytes"),
            MalformedContentLength => write!(f, "non-usize content length"),
            HeadersOverflow => write!(f, "the headers' total size surpassed max_headers_size"),
            StatusLineOverflow => write!(f, "the status line length surpassed max_status_line_length"),
            AddressNotFound => write!(f, "could not resolve host to a socket address"),
            RedirectLocationMissing => write!(f, "redirection location header missing"),
            InfiniteRedirectionLoop => write!(f, "infinite redirection loop detected"),
            TooManyRedirections => write!(f, "too many redirections (over the max)"),
            InvalidUtf8InResponse => write!(f, "response contained invalid utf-8 where valid utf-8 was expected"),
            HttpsFeatureNotEnabled => write!(f, "request url contains https:// but the https feature is not enabled"),
            NonASCIIurl => write!(f, "non-ascii urls not supported"),
            Other(msg) => write!(f, "error in minreq: please open an issue in the minreq repo, include the following: '{}'", msg),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            #[cfg(feature = "json-using-serde")]
            SerdeJsonError(err) => Some(err),
            IoError(err) => Some(err),
            InvalidUtf8InBody(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::IoError(other)
    }
}
