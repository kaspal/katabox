//! `Content-Length` framed JSON-RPC over a byte stream (LSP and DAP base protocol).

use std::io;

use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Largest message accepted (a hostile or broken peer cannot make us allocate more).
pub const MAX_MESSAGE: usize = 64 * 1024 * 1024;
/// Longest header line accepted.
const MAX_HEADER_LINE: usize = 8 * 1024;

fn invalid(msg: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg.into())
}

/// Read one message body. `Ok(None)` at a clean end of stream (between messages).
/// Header names are case-insensitive; headers other than `Content-Length` are ignored.
pub async fn read_message<R: AsyncBufRead + Unpin>(r: &mut R) -> io::Result<Option<Vec<u8>>> {
    let mut length: Option<usize> = None;
    let mut started = false;
    let mut line = Vec::new();
    loop {
        line.clear();
        let n = (&mut *r)
            .take(MAX_HEADER_LINE as u64 + 1)
            .read_until(b'\n', &mut line)
            .await?;
        if n == 0 {
            return if started {
                Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "stream ended inside a header",
                ))
            } else {
                Ok(None)
            };
        }
        if n > MAX_HEADER_LINE {
            return Err(invalid("header line too long"));
        }
        started = true;
        let text = std::str::from_utf8(&line).map_err(|_| invalid("header is not UTF-8"))?;
        let text = text.trim_end_matches(['\r', '\n']);
        if text.is_empty() {
            break;
        }
        let (name, value) = text
            .split_once(':')
            .ok_or_else(|| invalid(format!("malformed header {text:?}")))?;
        if name.trim().eq_ignore_ascii_case("content-length") {
            let len: usize = value
                .trim()
                .parse()
                .map_err(|_| invalid(format!("bad Content-Length {value:?}")))?;
            length = Some(len);
        }
    }
    let len = length.ok_or_else(|| invalid("missing Content-Length"))?;
    if len > MAX_MESSAGE {
        return Err(invalid(format!(
            "message of {len} bytes exceeds {MAX_MESSAGE}"
        )));
    }
    let mut body = vec![0; len];
    r.read_exact(&mut body).await?;
    Ok(Some(body))
}

/// Write one message with its `Content-Length` header and flush.
pub async fn write_message<W: AsyncWrite + Unpin>(w: &mut W, body: &[u8]) -> io::Result<()> {
    if body.len() > MAX_MESSAGE {
        return Err(invalid("message too large"));
    }
    w.write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())
        .await?;
    w.write_all(body).await?;
    w.flush().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::BufReader;

    async fn read_all(bytes: &[u8]) -> io::Result<Vec<Vec<u8>>> {
        let mut r = BufReader::new(bytes);
        let mut out = Vec::new();
        while let Some(m) = read_message(&mut r).await? {
            out.push(m);
        }
        Ok(out)
    }

    #[tokio::test]
    async fn reads_several_frames_and_ignores_other_headers() {
        let input = b"Content-Length: 2\r\n\r\n{}content-length: 7\r\nContent-Type: application/vscode-jsonrpc; charset=utf-8\r\n\r\n[1,2,3]";
        assert_eq!(
            read_all(input).await.unwrap(),
            [b"{}".to_vec(), b"[1,2,3]".to_vec()]
        );
        assert!(read_all(b"").await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn frames_split_across_reads() {
        let (mut tx, rx) = tokio::io::duplex(4);
        let writer = tokio::spawn(async move {
            write_message(&mut tx, br#"{"jsonrpc":"2.0","id":1}"#)
                .await
                .unwrap();
            write_message(&mut tx, "{\"x\":\"\u{e9}\"}".as_bytes())
                .await
                .unwrap();
        });
        let mut r = BufReader::new(rx);
        assert_eq!(
            read_message(&mut r).await.unwrap().unwrap(),
            br#"{"jsonrpc":"2.0","id":1}"#
        );
        // Length counts bytes, not characters.
        assert_eq!(
            read_message(&mut r).await.unwrap().unwrap(),
            "{\"x\":\"\u{e9}\"}".as_bytes()
        );
        writer.await.unwrap();
        assert!(read_message(&mut r).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn rejects_bad_input() {
        let kind = |r: io::Result<Vec<Vec<u8>>>| r.unwrap_err().kind();
        assert_eq!(
            kind(read_all(b"Content-Length: x\r\n\r\n").await),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            kind(read_all(b"Foo: 1\r\n\r\n{}").await),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            kind(read_all(b"no colon\r\n\r\n").await),
            io::ErrorKind::InvalidData
        );
        let huge = format!("Content-Length: {}\r\n\r\n", MAX_MESSAGE + 1);
        assert_eq!(
            kind(read_all(huge.as_bytes()).await),
            io::ErrorKind::InvalidData
        );
        let long = format!("X-{}: 1\r\n\r\n", "a".repeat(MAX_HEADER_LINE + 10));
        assert_eq!(
            kind(read_all(long.as_bytes()).await),
            io::ErrorKind::InvalidData
        );
        // Truncated body / header.
        assert_eq!(
            kind(read_all(b"Content-Length: 5\r\n\r\n{}").await),
            io::ErrorKind::UnexpectedEof
        );
        assert_eq!(
            kind(read_all(b"Content-Length: 5\r\n").await),
            io::ErrorKind::UnexpectedEof
        );
    }
}
