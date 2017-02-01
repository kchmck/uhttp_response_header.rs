# uhttp\_response\_header -- HTTP response header lines

[Documentation](https://docs.rs/uhttp_response_header)

This crate provides a simple formatter for building the lines of an [HTTP
response](https://tools.ietf.org/html/rfc7230#section-3) header. The result can be
written directly into a `TcpStream` or any other object that implements `Write`.

## Example

```rust
use uhttp_response_header::HeaderLines;
use std::io::{Cursor, Write};

let mut buf = [0; 40];
let mut cursor = Cursor::new(&mut buf[..]);

// Write a header with response code `200` and a `Host: iana.org` header field.
{
    let mut h = HeaderLines::new(&mut cursor);
    write!(h.line(), "{} {}", "HTTP/1.1", "200 OK").unwrap();
    write!(h.line(), "Host: {}", "iana.org").unwrap();
}

// Now write the body.
write!(&mut cursor, "hello").unwrap();

assert_eq!(cursor.into_inner(), &b"HTTP/1.1 200 OK\r\nHost: iana.org\r\n\r\nhello"[..]);
```

## Usage

This [crate](https://crates.io/crates/uhttp_response_header) can be used through cargo by
adding it as a dependency in `Cargo.toml`:

```toml
[dependencies]
uhttp_response_header = "0.5.0"
```
and importing it in the crate root:

```rust
extern crate uhttp_response_header;
```
