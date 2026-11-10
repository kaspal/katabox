//! The harness-to-judge protocol (ADR 0008).
//!
//! The judge sends one JSON request on stdin. The harness answers with records on stdout,
//! each on its own line as `<nonce> <json>`; the nonce is fresh per run so the user's own
//! output (which the harness captures anyway) can never be taken for a record.

use kb_core::MessageCode;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize)]
pub(crate) struct Request<'a> {
    pub nonce: &'a str,
    /// Cap on the user's captured prints per test, in characters.
    pub max_output_bytes: usize,
    pub tests: Vec<&'a Map<String, Value>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum Record {
    /// The solution does not parse (`syntax_error`, `unsupported_syntax`), or the
    /// function is missing (`function_not_found`).
    CompileError {
        code: MessageCode,
        message: String,
    },
    /// The solution parsed but raised while its module was loading.
    LoadError {
        message: String,
    },
    /// The solution loaded; test records follow.
    Ready,
    Result(TestRecord),
    /// Every test ran.
    Done,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct TestRecord {
    /// 0-based position in the request's `tests`.
    pub test: usize,
    pub status: Status,
    /// The return value (`ok` only).
    #[serde(default)]
    pub value: Value,
    /// `exception` or `return_not_serializable` (`error` only).
    #[serde(default)]
    pub error_code: Option<MessageCode>,
    /// Exception and traceback, or why the value is not JSON (`error` only).
    #[serde(default)]
    pub error: Option<String>,
    /// Wall time of the call in microseconds, measured inside the harness.
    pub time_us: u64,
    pub stdout: String,
    pub stdout_truncated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
    Ok,
    Error,
    Memory,
}

/// Records found in `stdout`, in order. Lines without the nonce are ignored; a line with
/// the nonce that does not parse is returned as `Err` (a harness bug).
pub(crate) fn parse(stdout: &[u8], nonce: &str) -> Vec<Result<Record, String>> {
    let prefix = format!("{nonce} ");
    String::from_utf8_lossy(stdout)
        .lines()
        .filter_map(|line| line.trim_end_matches('\r').strip_prefix(&prefix))
        .map(|json| serde_json::from_str(json).map_err(|e| format!("{e}: {json}")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_records_and_ignores_other_output() {
        let out = b"hello\n\nN1 {\"kind\":\"ready\"}\nnoise N1 {\"kind\":\"done\"}\n\
N1 {\"kind\":\"result\",\"test\":0,\"status\":\"ok\",\"value\":[0,1],\"time_us\":500,\"stdout\":\"hi\\n\",\"stdout_truncated\":false}\r\n\
N2 {\"kind\":\"done\"}\nN1 {\"kind\":\"done\"}\n";
        let recs: Vec<Record> = parse(out, "N1").into_iter().map(Result::unwrap).collect();
        assert_eq!(recs.len(), 3);
        assert_eq!(recs[0], Record::Ready);
        let Record::Result(t) = &recs[1] else {
            panic!("{recs:?}")
        };
        assert_eq!(t.value, json!([0, 1]));
        assert_eq!(t.status, Status::Ok);
        assert_eq!(t.stdout, "hi\n");
        assert_eq!(recs[2], Record::Done);
    }

    #[test]
    fn null_value_and_errors() {
        let out = b"N {\"kind\":\"result\",\"test\":1,\"status\":\"ok\",\"value\":null,\"time_us\":1,\"stdout\":\"\",\"stdout_truncated\":false}\n\
N {\"kind\":\"result\",\"test\":2,\"status\":\"error\",\"error_code\":\"exception\",\"error\":\"boom\",\"time_us\":1,\"stdout\":\"\",\"stdout_truncated\":false}\n\
N {\"kind\":\"bogus\"}\n";
        let recs = parse(out, "N");
        assert!(matches!(&recs[0], Ok(Record::Result(t)) if t.value.is_null()));
        assert!(matches!(&recs[1], Ok(Record::Result(t))
            if t.error.as_deref() == Some("boom") && t.error_code == Some(MessageCode::Exception)));
        assert!(recs[2].is_err());
    }
}
