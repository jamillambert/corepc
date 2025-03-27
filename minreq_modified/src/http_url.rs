extern crate alloc;

use alloc::fmt::{self, Write};

use crate::Error;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Port {
    ImplicitHttp,
    ImplicitHttps,
    Explicit(u32),
}

impl Port {
    pub(crate) fn port(self) -> u32 {
        match self {
            Port::ImplicitHttp => 80,
            Port::ImplicitHttps => 443,
            Port::Explicit(port) => port,
        }
    }
}

/// URL split into its parts. See [RFC 3986 section
/// 3](https://datatracker.ietf.org/doc/html/rfc3986#section-3). Note that the
/// userinfo component is not allowed since [RFC
/// 7230](https://datatracker.ietf.org/doc/html/rfc7230#section-2.7.1).
///
/// ```text
/// scheme "://" host [ ":" port ] path [ "?" query ] [ "#" fragment ]
/// ```
#[derive(Clone, PartialEq)]
pub(crate) struct HttpUrl {
    /// If scheme is "https", true, if "http", false.
    pub(crate) https: bool,
    /// `host`
    pub(crate) host: String,
    /// `[":" port]`
    pub(crate) port: Port,
    /// `path ["?" query]` including the `?`.
    pub(crate) path_and_query: String,
    /// `["#" fragment]` without the `#`.
    pub(crate) fragment: Option<String>,
}

impl HttpUrl {
    pub(crate) fn parse(url: &str, redirected_from: Option<&HttpUrl>) -> Result<HttpUrl, Error> {
        enum UrlParseStatus {
            Host,
            Port,
            PathAndQuery,
            Fragment,
        }

        let (url, https) = if let Some(after_protocol) = url.strip_prefix("http://") {
            (after_protocol, false)
        } else if let Some(after_protocol) = url.strip_prefix("https://") {
            (after_protocol, true)
        } else {
            // TODO: Uncomment this for 3.0
            // return Err(Error::InvalidProtocol);
            return Err(Error::IoError);
        };

        let mut host = String::new();
        let mut port = String::new();
        let mut resource = String::new(); // At first this is the path and query, after # this becomes fragment.
        let mut path_and_query = None;
        let mut status = UrlParseStatus::Host;
        for c in url.chars() {
            match status {
                UrlParseStatus::Host => {
                    match c {
                        '/' | '?' => {
                            // Tolerate typos like: www.example.com?some=params
                            status = UrlParseStatus::PathAndQuery;
                            resource.push(c);
                        }
                        ':' => status = UrlParseStatus::Port,
                        _ => host.push(c),
                    }
                }
                UrlParseStatus::Port => match c {
                    '/' | '?' => {
                        status = UrlParseStatus::PathAndQuery;
                        resource.push(c);
                    }
                    _ => port.push(c),
                },
                UrlParseStatus::PathAndQuery if c == '#' => {
                    status = UrlParseStatus::Fragment;
                    path_and_query = Some(resource);
                    resource = String::new();
                }
                UrlParseStatus::PathAndQuery | UrlParseStatus::Fragment => resource.push(c),
            }
        }
        let (mut path_and_query, mut fragment) = if let Some(path_and_query) = path_and_query {
            (path_and_query, Some(resource))
        } else {
            (resource, None)
        };

        // If a redirected resource does not have a fragment, but the original
        // URL did, the fragment should be preserved over redirections. See RFC
        // 7231 section 7.1.2.
        if fragment.is_none() {
            if let Some(old_fragment) = redirected_from.and_then(|url| url.fragment.clone()) {
                fragment = Some(old_fragment);
            }
        }

        // Ensure the resource is *something*
        if path_and_query.is_empty() {
            path_and_query.push('/');
        }

        // Set appropriate port
        let port = port.parse::<u32>().map(Port::Explicit).unwrap_or_else(|_| {
            if https {
                Port::ImplicitHttps
            } else {
                Port::ImplicitHttp
            }
        });

        Ok(HttpUrl {
            https,
            host,
            port,
            path_and_query,
            fragment,
        })
    }

    /// Writes the `scheme "://" host [ ":" port ]` part to the destination.
    pub(crate) fn write_base_url_to<W: Write>(&self, dst: &mut W) -> fmt::Result {
        write!(
            dst,
            "http{s}://{host}",
            s = if self.https { "s" } else { "" },
            host = &self.host,
        )?;
        if let Port::Explicit(port) = self.port {
            write!(dst, ":{}", port)?;
        }
        Ok(())
    }

    /// Writes the `path [ "?" query ] [ "#" fragment ]` part to the destination.
    pub(crate) fn write_resource_to<W: Write>(&self, dst: &mut W) -> fmt::Result {
        write!(
            dst,
            "{path_and_query}{maybe_hash}{maybe_fragment}",
            path_and_query = &self.path_and_query,
            maybe_hash = if self.fragment.is_some() { "#" } else { "" },
            maybe_fragment = self.fragment.as_deref().unwrap_or(""),
        )
    }
}
