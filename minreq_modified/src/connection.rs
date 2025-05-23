#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use crate::Error;

use core::time::Duration;
use core::result::Result;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::io::{self, Read, Write};
#[cfg(feature = "std")]
use std::net::{Shutdown, TcpStream};
#[cfg(feature = "std")]
use std::time::Instant;

#[cfg(not(feature = "std"))]
use core::ops::DerefMut;

/// Error type for no-std environments (replace as needed)
#[cfg(not(feature = "std"))]
#[derive(Debug)]
pub enum NoStdIoError {
    Other,
}

/// Trait for HTTP connections, supporting sync and async, generic over error type.
/// Works in both std and no-std environments.
pub trait HttpConnection<E>: Send + Sync {

    #[cfg(feature = "std")]
    /// Shutdown the connection.
    fn shutdown(&mut self, how: Shutdown) -> Result<(), E>;

    /// Synchronous read.
    fn sync_read(&mut self, buf: &mut [u8]) -> Result<usize, E>;

    /// Synchronous write.
    fn sync_write(&mut self, buf: &[u8]) -> Result<usize, E>;

    /// Synchronous flush.
    fn sync_flush(&mut self) -> Result<(), E>;

    /// Async read.
    #[cfg(feature = "async")]
    fn poll_read<'a>(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
        buf: &mut [u8],
    ) -> core::task::Poll<Result<usize, E>> {
        let _ = (cx, buf);
        core::task::Poll::Pending
    }

    /// Async write.
    #[cfg(feature = "async")]
    fn poll_write<'a>(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
        buf: &[u8],
    ) -> core::task::Poll<Result<usize, E>> {
        let _ = (cx, buf);
        core::task::Poll::Pending
    }

    /// Async flush.
    #[cfg(feature = "async")]
    fn poll_flush<'a>(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), E>> {
        let _ = cx;
        core::task::Poll::Pending
    }

    /// Async shutdown.
    #[cfg(feature = "async")]
    fn poll_shutdown<'a>(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), E>> {
        let _ = cx;
        core::task::Poll::Pending
    }
}

#[cfg(feature = "std")]
impl HttpConnection<io::Error> for TcpStream {
    fn shutdown(&mut self, how: Shutdown) -> Result<(), io::Error> {
        TcpStream::shutdown(self, how)
    }

    fn sync_read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.read(buf)
    }

    fn sync_write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.write(buf)
    }

    fn sync_flush(&mut self) -> Result<(), io::Error> {
        self.flush()
    }

    // Do NOT try to use tokio's AsyncRead/AsyncWrite for std::net::TcpStream!
    #[cfg(feature = "async")]
    fn poll_read<'a>(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        _buf: &mut [u8],
    ) -> std::task::Poll<Result<usize, io::Error>> {
        std::task::Poll::Pending
    }

    #[cfg(feature = "async")]
    fn poll_write<'a>(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        _buf: &[u8],
    ) -> std::task::Poll<Result<usize, io::Error>> {
        std::task::Poll::Pending
    }

    #[cfg(feature = "async")]
    fn poll_flush<'a>(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        std::task::Poll::Pending
    }

    #[cfg(feature = "async")]
    fn poll_shutdown<'a>(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        std::task::Poll::Pending
    }
}

pub(crate) struct HttpStream {
    inner: Box<dyn HttpConnection<io::Error>>,
    timeout_at: Option<Instant>,
}

fn timeout_err() -> io::Error {
    io::Error::new(
        io::ErrorKind::TimedOut,
        "the timeout of the request was reached",
    )
}

fn timeout_at_to_duration(timeout_at: Option<Instant>) -> Result<Option<Duration>, io::Error> {
    if let Some(timeout_at) = timeout_at {
        if let Some(duration) = timeout_at.checked_duration_since(Instant::now()) {
            Ok(Some(duration))
        } else {
            Err(timeout_err())
        }
    } else {
        Ok(None)
    }
}

impl Read for HttpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.sync_read(buf)
    }
}

impl Write for HttpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.sync_write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.sync_flush()
    }
}

impl HttpStream {
    #[allow(dead_code)]
    fn shutdown(&mut self, how: Shutdown) -> io::Result<()> {
        self.inner.shutdown(how)
    }

    /// Write all bytes from the buffer, using sync_write repeatedly.
    pub fn write_all(&mut self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.inner.sync_write(buf) {
                Ok(0) => return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write whole buffer")),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/// Enforce the timeout by running the function in a new thread and
/// parking the current one with a timeout.
///
/// While minreq does use timeouts (somewhat) properly, some
/// interfaces such as [ToSocketAddrs] don't allow for specifying the
/// timeout. Hence this.
fn enforce_timeout<F, R>(timeout_at: Option<Instant>, f: F) -> Result<R, Error>
where
    F: 'static + Send + FnOnce() -> Result<R, Error>,
    R: 'static + Send,
{
    use std::sync::mpsc::{channel, RecvTimeoutError};

    match timeout_at {
        Some(deadline) => {
            let (sender, receiver) = channel();
            let thread = std::thread::spawn(move || {
                let result = f();
                let _ = sender.send(());
                result
            });
            if let Some(timeout_duration) = deadline.checked_duration_since(Instant::now()) {
                match receiver.recv_timeout(timeout_duration) {
                    Ok(()) => thread.join().unwrap(),
                    Err(err) => match err {
                        RecvTimeoutError::Timeout => Err(Error::IoError),
                        RecvTimeoutError::Disconnected => {
                            Err(Error::Other("request connection paniced"))
                        }
                    },
                }
            } else {
                Err(Error::IoError)
            }
        }
        None => f(),
    }
}

fn ensure_ascii_host(host: String) -> Result<String, Error> {
    if host.is_ascii() {
        Ok(host)
    } else {
        Err(Error::NonASCIIurl)
    }
}

#[cfg(feature = "std")]
pub mod std_connection {
    use super::*;
    use std::io;
    use std::net::{TcpStream, ToSocketAddrs};
    use std::time::Instant;
    use core::time::Duration;
    use crate::request::ParsedRequest;
    use crate::{Error, Method, ResponseLazy};

    /// A connection to the server for sending
    /// [`Request`](struct.Request.html)s.
    pub struct Connection {
        request: ParsedRequest,
        timeout_at: Option<Instant>,
    }

    impl Connection {
        /// Creates a new `Connection`. See [Request] and [ParsedRequest]
        /// for specifics about *what* is being sent.
        pub(crate) fn new(request: ParsedRequest) -> Connection {
            let timeout = request.config.timeout;
            let timeout_at = timeout.map(|t| Instant::now() + Duration::from_secs(t));
            Connection {
                request,
                timeout_at,
            }
        }

        /// Returns the timeout duration for operations that should end at
        /// timeout and are starting "now".
        ///
        /// The Result will be Err if the timeout has already passed.
        fn timeout(&self) -> Result<Option<Duration>, io::Error> {
            let timeout = super::timeout_at_to_duration(self.timeout_at);
            log::trace!("Timeout requested, it is currently: {:?}", timeout);
            timeout
        }

        /// Sends the [`Request`](struct.Request.html), consumes this
        /// connection, and returns a [`Response`](struct.Response.html).
        pub(crate) fn send(mut self) -> Result<ResponseLazy, Error> {
            super::enforce_timeout(self.timeout_at, move || {
                self.request.url.host = super::ensure_ascii_host(self.request.url.host)?;
                let bytes = self.request.as_bytes();

                log::trace!("Establishing TCP connection to {}.", self.request.url.host);
                let conn = self.connect()?;

                // Send request using sync_write_all
                log::trace!("Writing HTTP request.");
                {
                    let mut http_stream = super::HttpStream {
                        inner: conn,
                        timeout_at: self.timeout_at,
                    };
                    http_stream.write_all(&bytes).map_err(|_| Error::IoError)?;
                    // Receive response
                    log::trace!("Reading HTTP response.");
                    let response = ResponseLazy::from_stream(
                        http_stream,
                        self.request.config.max_headers_size,
                        self.request.config.max_status_line_len,
                    )?;
                    return handle_redirects(self, response);
                }
            })
        }

        /// Connects and returns a boxed trait object implementing HttpConnection.
        fn connect(&self) -> Result<Box<dyn HttpConnection<io::Error>>, Error> {
            let tcp_connect = |host: &str, port: u32| -> Result<TcpStream, Error> {
                let addrs = (host, port as u16)
                    .to_socket_addrs()
                    .map_err(|_| Error::IoError).unwrap();
                let addrs_count = addrs.len();

                // Try all resolved addresses. Return the first one to which we could connect. If all
                // failed return the last error encountered.
                for (i, addr) in addrs.enumerate() {
                    let stream = if let Some(timeout) = self.timeout().unwrap() {
                        TcpStream::connect_timeout(&addr, timeout)
                    } else {
                        TcpStream::connect(addr)
                    };
                    if stream.is_ok() || i == addrs_count - 1 {
                        return stream.map_err(|_| Error::IoError);
                    }
                }

                Err(Error::AddressNotFound)
            };

            let tcp = tcp_connect(&self.request.url.host, self.request.url.port.port())?;
            Ok(Box::new(tcp) as Box<dyn HttpConnection<io::Error>>)
        }
    }

    fn handle_redirects(
        connection: Connection,
        mut response: ResponseLazy,
    ) -> Result<ResponseLazy, Error> {
        let status_code = response.status_code;
        let url = response.headers.get("location");
        match get_redirect(connection, status_code, url) {
            NextHop::Redirect(connection) => {
                let connection = connection?;
                connection.send()
            }
            NextHop::Destination(connection) => {
                let dst_url = connection.request.url;
                dst_url.write_base_url_to(&mut response.url).unwrap();
                dst_url.write_resource_to(&mut response.url).unwrap();
                Ok(response)
            }
        }
    }

    enum NextHop {
        Redirect(Result<Connection, Error>),
        Destination(Connection),
    }

    fn get_redirect(mut connection: Connection, status_code: i32, url: Option<&String>) -> NextHop {
        match status_code {
            301 | 302 | 303 | 307 => {
                let url = match url {
                    Some(url) => url,
                    None => return NextHop::Redirect(Err(Error::RedirectLocationMissing)),
                };
                log::debug!("Redirecting ({}) to: {}", status_code, url);

                match connection.request.redirect_to(url.as_str()) {
                    Ok(()) => {
                        if status_code == 303 {
                            match connection.request.config.method {
                                Method::Post | Method::Put | Method::Delete => {
                                    connection.request.config.method = Method::Get;
                                }
                                _ => {}
                            }
                        }

                        NextHop::Redirect(Ok(connection))
                    }
                    Err(err) => NextHop::Redirect(Err(err)),
                }
            }
            _ => NextHop::Destination(connection),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream, Shutdown};
    use std::thread;

    #[test]
    fn test_sync_read_write_flush_shutdown() {
        // Start a local TCP server
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn a thread to accept the connection and echo data
        thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut buf = [0u8; 128];
            let n = socket.read(&mut buf).unwrap();
            socket.write_all(&buf[..n]).unwrap();
            socket.flush().unwrap();
            socket.shutdown(Shutdown::Both).unwrap();
        });

        // Connect to the server
        let mut stream = TcpStream::connect(addr).unwrap();

        // Test sync_write
        let msg = b"hello world";
        let n = <TcpStream as HttpConnection<io::Error>>::sync_write(&mut stream, msg).unwrap();
        assert_eq!(n, msg.len());

        // Test sync_flush
        <TcpStream as HttpConnection<io::Error>>::sync_flush(&mut stream).unwrap();

        // Test sync_read
        let mut buf = [0u8; 128];
        let n = <TcpStream as HttpConnection<io::Error>>::sync_read(&mut stream, &mut buf).unwrap();
        assert_eq!(&buf[..n], msg);

        // Test shutdown
        <TcpStream as HttpConnection<io::Error>>::shutdown(&mut stream, Shutdown::Both).unwrap();
    }

    #[test]
    fn test_httpstream_sync_traits() {
        // Start a local TCP server
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut buf = [0u8; 128];
            let n = socket.read(&mut buf).unwrap();
            socket.write_all(&buf[..n]).unwrap();
        });

        let stream = TcpStream::connect(addr).unwrap();
        let mut http_stream = HttpStream {
            inner: Box::new(stream),
            timeout_at: None,
        };

        // Test Write for HttpStream
        let msg = b"ping";
        let n = http_stream.write(msg).unwrap();
        assert_eq!(n, msg.len());

        // Test Flush for HttpStream
        http_stream.flush().unwrap();

        // Test Read for HttpStream
        let mut buf = [0u8; 128];
        let n = http_stream.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], msg);
    }

    #[test]
    fn test_ensure_ascii_host() {
        assert!(ensure_ascii_host("ascii.com".to_string()).is_ok());
        assert!(ensure_ascii_host("unicodé.com".to_string()).is_err());
    }
}
