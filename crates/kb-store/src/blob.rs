//! Compressed JSON for test data in content.db (ADR 0018): each test's input and output is
//! JSON compressed with zstd, per test, so public tests load without the hidden ones.

use std::io::Read;

use kb_core::TestCase;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{Result, StoreError};

/// zstd level used when building content.db (slow to compress, fast to read).
pub const LEVEL: i32 = 19;

/// Largest decompressed value accepted (one test's input or output), so a corrupt or
/// hostile pack cannot exhaust memory.
pub const MAX_DECOMPRESSED: u64 = 64 * 1024 * 1024;

/// `value` as compact JSON, compressed.
pub fn compress_json(value: &impl Serialize) -> Result<Vec<u8>> {
    let json = serde_json::to_vec(value)?;
    Ok(zstd::bulk::compress(&json, LEVEL)?)
}

/// The JSON value in a blob made by [`compress_json`].
pub fn decompress_json<T: DeserializeOwned>(blob: &[u8]) -> Result<T> {
    let mut json = Vec::new();
    zstd::stream::read::Decoder::new(blob)
        .map_err(|e| StoreError::Corrupt(format!("test data: {e}")))?
        .take(MAX_DECOMPRESSED + 1)
        .read_to_end(&mut json)
        .map_err(|e| StoreError::Corrupt(format!("test data: {e}")))?;
    if json.len() as u64 > MAX_DECOMPRESSED {
        return Err(StoreError::Corrupt(format!(
            "test data larger than {MAX_DECOMPRESSED} bytes"
        )));
    }
    Ok(serde_json::from_slice(&json)?)
}

/// Bytes of a problem's tests: compact JSON, and as stored in content.db (input and
/// output compressed separately). kb-pack checks the stored size against its budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TestBytes {
    pub json: u64,
    pub stored: u64,
}

pub fn test_bytes(tests: &[TestCase]) -> Result<TestBytes> {
    let mut b = TestBytes::default();
    for t in tests {
        for v in [
            serde_json::Value::Object(t.input.clone()),
            t.expected.clone(),
        ] {
            b.json += serde_json::to_vec(&v)?.len() as u64;
            b.stored += compress_json(&v)?.len() as u64;
        }
    }
    Ok(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::TestKind;
    use serde_json::{Value, json};

    #[test]
    fn round_trip_and_smaller_for_large_values() {
        let big: Vec<i64> = (0..20_000).map(|i| (i * 7919) % 1000).collect();
        let v = json!({ "nums": big });
        let blob = compress_json(&v).unwrap();
        assert!(blob.len() * 3 < serde_json::to_vec(&v).unwrap().len());
        assert_eq!(decompress_json::<Value>(&blob).unwrap(), v);
    }

    #[test]
    fn corrupt_data_is_an_error() {
        let err = decompress_json::<Value>(b"not zstd at all").unwrap_err();
        assert!(matches!(err, StoreError::Corrupt(_)), "{err}");
        let mut blob = compress_json(&json!([1, 2, 3])).unwrap();
        blob.truncate(blob.len() - 2);
        assert!(decompress_json::<Value>(&blob).is_err());
    }

    #[test]
    fn oversized_data_is_refused() {
        // A tiny blob that expands past the cap (a "zip bomb").
        let huge = vec![b' '; (MAX_DECOMPRESSED + 10) as usize];
        let blob = zstd::bulk::compress(&huge, 3).unwrap();
        assert!(blob.len() < 1024 * 1024);
        let err = decompress_json::<Value>(&blob).unwrap_err();
        assert!(err.to_string().contains("larger than"), "{err}");
    }

    #[test]
    fn test_bytes_counts_input_and_output() {
        let t = TestCase {
            kind: TestKind::Hidden,
            index: 1,
            input: json!({"a": [1, 2]}).as_object().unwrap().clone(),
            expected: json!(3),
        };
        let b = test_bytes(&[t.clone(), t]).unwrap();
        assert_eq!(b.json, 2 * (r#"{"a":[1,2]}"#.len() + 1) as u64);
        assert!(b.stored > 0);
    }
}
