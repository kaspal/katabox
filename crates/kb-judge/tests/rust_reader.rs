//! The Rust harness's JSON reader (`languages/rust/json.rs`, ADR 0026), tested as shipped:
//! the file is `include!`d here exactly as kb-judge injects it into every Rust harness.
//!
//! - Differential: every test file of every problem, and whole requests, read by the
//!   reader and compared with serde_json's view of the same text.
//! - Typed fast path: every test input, read straight into its signature's Rust type,
//!   equals the generic path (`parse` + `FromJson`), and the fast path is really taken.
//! - Edge cases, and malformed input that must fail cleanly (exit 70), never panic.

#[allow(dead_code, clippy::all)]
mod reader {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../languages/rust/json.rs"
    ));
}

use reader::{FromJson, Json, MAX_DEPTH, Parser, ReadJson, parse, read_value};
use serde_json::Value;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

fn problems_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems")
}

/// The reader's value as serde_json sees values. Duplicate keys keep the last value, as
/// serde_json does (test files never repeat a key).
fn to_value(v: &Json) -> Value {
    match v {
        Json::Null => Value::Null,
        Json::Bool(b) => Value::Bool(*b),
        Json::Int(n) => Value::from(*n),
        Json::Num(text) => serde_json::from_str(text).expect("a number kept as written"),
        Json::Str(s) => Value::String(s.clone()),
        Json::Arr(items) => Value::Array(items.iter().map(to_value).collect()),
        Json::Obj(fields) => {
            let mut map = serde_json::Map::new();
            for (k, v) in fields {
                map.insert(k.clone(), to_value(v));
            }
            Value::Object(map)
        }
    }
}

fn same_as_serde(text: &str) {
    let ours = parse(text).unwrap_or_else(|e| panic!("reader rejects {text:.80}: {e}"));
    let theirs: Value = serde_json::from_str(text).expect("serde_json reads it");
    assert_eq!(to_value(&ours), theirs, "{text:.200}");
}

fn test_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for kind in ["public", "hidden"] {
        if let Ok(entries) = std::fs::read_dir(dir.join("tests").join(kind)) {
            files.extend(entries.map(|e| e.unwrap().path()));
        }
    }
    files.sort();
    files
}

fn problem_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(problems_dir())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.join("problem.toml").is_file())
        .collect();
    dirs.sort();
    assert!(dirs.len() >= 150, "found {} problems", dirs.len());
    dirs
}

#[test]
fn every_test_file_reads_as_serde_json_does() {
    let mut files = 0;
    for dir in problem_dirs() {
        for file in test_files(&dir) {
            same_as_serde(&std::fs::read_to_string(&file).unwrap());
            files += 1;
        }
    }
    assert!(files > 3000, "only {files} test files");
}

/// The request kb-judge sends: its `Request` struct, serialised compactly.
fn request(dir: &Path) -> String {
    let inputs: Vec<Value> = test_files(dir)
        .iter()
        .map(|f| {
            let v: Value = serde_json::from_str(&std::fs::read_to_string(f).unwrap()).unwrap();
            v["input"].clone()
        })
        .collect();
    format!(
        "{{\"nonce\":\"a1b2c3\",\"max_output_bytes\":65536,\"tests\":{}}}",
        serde_json::to_string(&inputs).unwrap()
    )
}

#[test]
fn whole_requests_read_as_serde_json_does() {
    // Every tenth problem: the inputs are the files above, this adds the outer object.
    for dir in problem_dirs().iter().step_by(10) {
        same_as_serde(&request(dir));
    }
}

/// Reads `text` as `T` on the fast path, and checks it against the generic path.
fn typed<T: ReadJson + FromJson + PartialEq + Debug>(text: &str) {
    let generic = T::from_json(&parse(text).unwrap());
    let mut p = Parser::new(text);
    match T::read(&mut p) {
        Ok(fast) => assert_eq!(Ok(&fast), generic.as_ref(), "{text:.200}"),
        Err(_) => panic!("the fast path did not take {text:.200}"),
    }
    let mut p = Parser::new(text);
    assert_eq!(read_value::<T>(&mut p).unwrap(), generic, "{text:.200}");
}

#[test]
fn every_test_input_takes_the_fast_path_with_the_generic_result() {
    let mut checked = 0;
    for dir in problem_dirs() {
        let problem = kb_judge::Problem::load(&dir).unwrap();
        for file in test_files(&dir) {
            let v: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
            for param in &problem.signature.params {
                let text = serde_json::to_string(&v["input"][&param.name]).unwrap();
                match param.ty.to_string().as_str() {
                    "int" => typed::<i64>(&text),
                    "float" => typed::<f64>(&text),
                    "bool" => typed::<bool>(&text),
                    "string" => typed::<String>(&text),
                    "int[]" => typed::<Vec<i64>>(&text),
                    "float[]" => typed::<Vec<f64>>(&text),
                    "string[]" => typed::<Vec<String>>(&text),
                    "int[][]" => typed::<Vec<Vec<i64>>>(&text),
                    // Node types read through the generic path (template code).
                    "ListNode" | "TreeNode" | "ListNode[]" | "TreeNode[]" => continue,
                    other => panic!("{}: add a typed check for {other}", dir.display()),
                }
                checked += 1;
            }
        }
    }
    assert!(checked > 5000, "only {checked} typed values");
}

/// A value of the wrong type: the fast path declines and the generic path decides, with
/// the generic message.
fn mismatch<T: ReadJson + FromJson + PartialEq + Debug>(text: &str) {
    let generic = T::from_json(&parse(text).unwrap());
    let mut p = Parser::new(text);
    assert_eq!(read_value::<T>(&mut p).unwrap(), generic, "{text}");
}

#[test]
fn values_of_the_wrong_type_get_the_generic_answer() {
    for text in [
        "null",
        "true",
        "\"7\"",
        "7",
        "-7",
        "2.5",
        "1e3",
        "3.0",
        "+4",
        "[]",
        "[1,2]",
        "[[1],[2,3]]",
        "[1,\"x\"]",
        "[1,null]",
        "[[1],2]",
        "{}",
        "{\"a\":1}",
        "[true]",
        "[\"a\",\"b\"]",
        "[2.5, 3]",
        "9007199254740991",
        "-9007199254740991",
        "99999999999999999999",
    ] {
        mismatch::<i64>(text);
        mismatch::<f64>(text);
        mismatch::<bool>(text);
        mismatch::<String>(text);
        mismatch::<Vec<i64>>(text);
        mismatch::<Vec<f64>>(text);
        mismatch::<Vec<String>>(text);
        mismatch::<Vec<Vec<i64>>>(text);
    }
    // The generic path's semantics, kept exactly: integral floats are integers.
    assert_eq!(i64::from_json(&parse("3.0").unwrap()), Ok(3));
    assert_eq!(i64::from_json(&parse("1e3").unwrap()), Ok(1000));
    assert_eq!(i64::from_json(&parse("+4").unwrap()), Ok(4));
    assert!(i64::from_json(&parse("2.5").unwrap()).is_err());
    assert_eq!(f64::from_json(&parse("7").unwrap()), Ok(7.0));
    // Messages show values as the earlier reader did (`Num("7")`, never `Int(7)`).
    assert_eq!(
        String::from_json(&parse("7").unwrap()),
        Err("expected a string, got Num(\"7\")".to_string())
    );
    assert_eq!(
        Vec::<i64>::from_json(&parse("\"oops\"").unwrap()),
        Err("expected an array, got Str(\"oops\")".to_string())
    );
}

#[test]
fn edge_cases_read_as_serde_json_does() {
    let safe = "9007199254740991";
    for text in [
        // Escapes, \u, surrogate pairs, non-ASCII.
        r#""\" \\ \/ \b \f \n \r \t""#,
        r#""\u0041\u00e9\u4e2d\ud83d\ude00""#,
        "\"héllo wörld 中文 😀\"",
        r#"{"k\u00e9y":"v\"al"}"#,
        // Numbers (`-0` is checked below: serde_json reads it as the float -0.0).
        "0",
        "7",
        "-7",
        "1e3",
        "1E3",
        "2.5E-3",
        "-2.5e+3",
        "0.1",
        "123456789012345678",
        "-123456789012345678",
        "1234567890123456789",
        "9223372036854775807",
        "-9223372036854775808",
        "18446744073709551615",
        "1.7976931348623157e308",
        // Containers.
        "[]",
        "{}",
        "[[]]",
        "[{}]",
        "{\"a\":[]}",
        "[[[[[[[[1]]]]]]]]",
        "[null,[null],{}]",
        // ListNode[] with an empty list, and a tree with nulls.
        "[[1,2],[],[3]]",
        "[1,null,2,null,null,3]",
        // Whitespace everywhere.
        " \t\n\r{ \"a\" : [ 1 , 2 ] ,\n \"b\" :\t\"x\" } \r\n",
    ] {
        same_as_serde(text);
    }
    same_as_serde(safe);
    same_as_serde(&format!("-{safe}"));
    assert!(matches!(
        parse(safe).unwrap(),
        Json::Int(9_007_199_254_740_991)
    ));
    assert!(matches!(
        parse(&format!("-{safe}")).unwrap(),
        Json::Int(-9_007_199_254_740_991)
    ));
    // `-0` has always been the integer 0 for the harness.
    assert!(matches!(parse("-0").unwrap(), Json::Int(0)));
    assert_eq!(i64::from_json(&parse("-0").unwrap()), Ok(0));
    // A lone surrogate becomes U+FFFD, as before (serde_json rejects it).
    assert!(matches!(parse(r#""\udc00""#).unwrap(), Json::Str(s) if s == "\u{FFFD}"));
}

#[test]
fn nesting_up_to_the_limit_reads_and_beyond_it_fails_cleanly() {
    // The harness reads on a thread with a 64 MB stack; so does this test.
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(|| {
            let nested = |n: usize| format!("{}{}", "[".repeat(n), "]".repeat(n));
            assert!(parse(&nested(MAX_DEPTH)).is_ok());
            assert!(parse(&nested(MAX_DEPTH + 1)).is_err());
            let too_deep = nested(MAX_DEPTH + 1);
            let mut p = Parser::new(&too_deep);
            assert!(read_value::<Vec<Vec<i64>>>(&mut p).is_err());
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn malformed_input_is_an_error_never_a_panic() {
    for text in [
        "",
        " ",
        "nul",
        "tru",
        "[",
        "]",
        "{",
        "}",
        "[1,]",
        "[,1]",
        "{\"a\"}",
        "{\"a\":}",
        "{\"a\" 1}",
        "{1:2}",
        "\"abc",
        "\"\\x\"",
        "\"\\u12\"",
        "\"\\uZZZZ\"",
        "\"\\ud800\"",
        "\"\\ud800\\u0041\"",
        "[1 2]",
        "{} {}",
        "1 2",
        "[\"a\" \"b\"]",
        "x",
        "[1]x",
        "\u{0}",
        "{\"a\":1,}",
    ] {
        if text == "\"\\ud800\\u0041\"" {
            // A high surrogate followed by a non-surrogate is replaced, as before.
            assert!(parse(text).is_ok());
            continue;
        }
        assert!(parse(text).is_err(), "accepted {text:?}");
        // The typed reader reads one value of a larger document (the request reader
        // checks what follows), so it must only never panic here...
        let mut p = Parser::new(text);
        let _ = read_value::<Vec<i64>>(&mut p);
    }
    // Number tokens stay as lenient as before: a run of number characters is read as
    // written, and only converting it can fail.
    assert!(matches!(parse("-").unwrap(), Json::Num(n) if n == "-"));
    assert!(i64::from_json(&parse("-").unwrap()).is_err());
    assert!(f64::from_json(&parse("1.2.3").unwrap()).is_err());
    // ...and must fail when the value itself is broken.
    for text in [
        "[",
        "[1,",
        "[1 2]",
        "[1,]",
        "[,1]",
        "[\"a]",
        "[1,[2",
        "{\"a\":",
        "[1,\"\\x\"]",
    ] {
        let mut p = Parser::new(text);
        assert!(
            read_value::<Vec<i64>>(&mut p).is_err(),
            "typed accepted {text:?}"
        );
        let mut p = Parser::new(text);
        assert!(
            read_value::<Vec<Vec<i64>>>(&mut p).is_err(),
            "typed accepted {text:?}"
        );
    }
    // Every proper prefix of real requests and test files.
    for dir in problem_dirs().iter().step_by(15) {
        for file in test_files(dir).iter().take(3) {
            let text = std::fs::read_to_string(file).unwrap();
            let text = text.trim_end();
            for end in 0..text.len() {
                if text.is_char_boundary(end) {
                    assert!(
                        parse(&text[..end]).is_err(),
                        "{}: prefix {end}",
                        file.display()
                    );
                }
            }
        }
    }
}
