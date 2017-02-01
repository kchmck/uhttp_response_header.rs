//! This crate provides a simple formatter for building the lines of an [HTTP
//! response](https://tools.ietf.org/html/rfc7230#section-3) header. The result can be
//! written directly into a `TcpStream` or any other object that implements `Write`.
//!
//! ## Example
//!
//! ```rust
//! use uhttp_response_header::HeaderLines;
//! use std::io::{Cursor, Write};
//!
//! let mut buf = [0; 40];
//! let mut cursor = Cursor::new(&mut buf[..]);
//!
//! // Write a header with response code `200` and a `Host: iana.org` header field.
//! {
//!     let mut h = HeaderLines::new(&mut cursor);
//!     write!(h.line(), "{} {}", "HTTP/1.1", "200 OK").unwrap();
//!     write!(h.line(), "Host: {}", "iana.org").unwrap();
//! }
//!
//! // Now write the body.
//! write!(&mut cursor, "hello").unwrap();
//!
//! assert_eq!(cursor.into_inner(), &b"HTTP/1.1 200 OK\r\nHost: iana.org\r\n\r\nhello"[..]);
//! ```

use std::io::Write;

/// Writes out the lines in an HTTP response header.
///
/// A response header is made of any number of lines, each terminated by a CRLF, followed
/// by a final terminating CRLF before the response body begins.
///
/// When this object goes out of scope the header is terminated and the stream is flushed.
pub struct HeaderLines<W: Write>(W);

impl<W: Write> HeaderLines<W> {
    /// Create a new `HeaderLines` writing into the given stream.
    pub fn new(sink: W) -> Self {
        HeaderLines(sink)
    }

    /// Add a new line to the header, which can be written into.
    pub fn line(&mut self) -> HeaderLine<&mut W> {
        HeaderLine(&mut self.0)
    }
}

impl<W: Write> Drop for HeaderLines<W> {
    fn drop(&mut self) {
        // Output an empty line and flush the buffer.
        self.line();
        self.0.flush().is_ok();
    }
}

/// Writes out a header line.
///
/// When this object goes out of scope the line is terminated. The string written into the
/// line must not contain any CRLFs (`\r\n`.)
pub struct HeaderLine<W: Write>(W);

impl<W: Write> Write for HeaderLine<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.0.write(buf) }
    fn flush(&mut self) -> std::io::Result<()> { self.0.flush() }
}

impl<W: Write> Drop for HeaderLine<W> {
    fn drop(&mut self) { self.write(&b"\r\n"[..]).is_ok(); }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_line() {
        let mut buf = [0u8; 13];

        {
            let mut c = Cursor::new(&mut buf[..]);
            let mut h = HeaderLine(&mut c);

            write!(&mut h, "ABC: DEF").unwrap();
            write!(&mut h, " {}", 42).unwrap();
        }

        assert_eq!(&buf[..], b"ABC: DEF 42\r\n");
    }

    #[test]
    fn test_header_lines() {
        let mut buf = [0u8; 30];

        {
            let mut c = Cursor::new(&mut buf[..]);
            let mut h = HeaderLines::new(&mut c);

            write!(h.line(), "header: value").unwrap();
            write!(h.line(), "field: {}", 1337).unwrap();
        }

        assert_eq!(&buf[..], b"header: value\r\nfield: 1337\r\n\r\n");
    }
}
