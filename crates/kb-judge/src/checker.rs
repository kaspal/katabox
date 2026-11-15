//! Built-in checkers. Function problems compare JSON values, so every language's
//! harness only has to serialize its return value. Program problems (ADR 0028) compare
//! the program's stdout text with the expected text: by tokens, by tokens with a float
//! tolerance, or by lines.

use kb_core::{Message, MessageCode};
use serde_json::Value;
use std::str::FromStr;

/// How a problem's output is compared, from `[judge] checker = "..."` in problem.toml.
#[derive(Debug, Clone, PartialEq)]
pub enum Checker {
    /// Exact JSON equality.
    Exact,
    /// Top-level arrays compared as multisets (order ignored).
    Unordered,
    /// Numbers compared with an absolute-or-relative tolerance, recursively.
    Float(f64),
    /// A custom checker script shipped in the pack (run by the runner, not here).
    Custom,
    /// Program problems: tokens separated by ASCII whitespace must be equal.
    Tokens,
    /// Program problems: like `Tokens`, but numeric tokens may differ by an absolute or
    /// relative tolerance.
    TokensFloat(f64),
    /// Program problems: lines must be equal after removing trailing spaces and tabs;
    /// trailing empty lines (the final newline) do not count.
    Lines,
    /// SQL problems (ADR 0031): `{"columns", "rows"}` results. Column names must match
    /// (case-insensitively, in order); rows in order (`ordered`) or as a multiset.
    /// Numbers compare by value (2 equals 2.0), within `eps` when given; text exactly;
    /// NULL only equals NULL.
    Rows { ordered: bool, eps: Option<f64> },
}

impl FromStr for Checker {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exact" => Ok(Checker::Exact),
            "unordered" => Ok(Checker::Unordered),
            "custom" => Ok(Checker::Custom),
            "tokens" => Ok(Checker::Tokens),
            "lines" => Ok(Checker::Lines),
            _ => {
                let tolerance = |eps: &str| -> Result<f64, String> {
                    let eps: f64 = eps.parse().map_err(|_| format!("bad tolerance in {s:?}"))?;
                    if eps > 0.0 && eps.is_finite() {
                        Ok(eps)
                    } else {
                        Err(format!("bad tolerance in {s:?}"))
                    }
                };
                if let Some(eps) = s.strip_prefix("tokens-float:") {
                    return tolerance(eps).map(Checker::TokensFloat);
                }
                if let Some(eps) = s.strip_prefix("float:") {
                    return tolerance(eps).map(Checker::Float);
                }
                for (prefix, ordered) in [("rows-ordered", true), ("rows-unordered", false)] {
                    if let Some(rest) = s.strip_prefix(prefix) {
                        if rest.is_empty() {
                            return Ok(Checker::Rows { ordered, eps: None });
                        }
                        if let Some(eps) = rest.strip_prefix(':') {
                            return tolerance(eps).map(|e| Checker::Rows {
                                ordered,
                                eps: Some(e),
                            });
                        }
                    }
                }
                Err(format!(
                    "unknown checker {s:?} (expected exact | unordered | float:<eps> | custom | \
                     tokens | tokens-float:<eps> | lines | rows-ordered[:<eps>] | \
                     rows-unordered[:<eps>])"
                ))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CheckOutcome {
    Ok,
    Wrong(Message),
}

impl Checker {
    /// Whether this checker compares program output text (program problems only).
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            Checker::Tokens | Checker::TokensFloat(_) | Checker::Lines
        )
    }

    /// Whether this checker compares SQL results (SQL problems only).
    pub fn is_rows(&self) -> bool {
        matches!(self, Checker::Rows { .. })
    }

    /// Compare `actual` against `expected`. `Custom` always returns an error message
    /// because it must be executed by the runner. Text checkers expect JSON strings (the
    /// test's expected stdout and the program's stdout).
    pub fn check(&self, expected: &Value, actual: &Value) -> CheckOutcome {
        if self.is_text() {
            let (Value::String(e), Value::String(a)) = (expected, actual) else {
                return CheckOutcome::Wrong(Message::new(
                    MessageCode::OutputMismatch,
                    "expected text output",
                ));
            };
            return match self.check_text(e, a) {
                None => CheckOutcome::Ok,
                Some(why) => CheckOutcome::Wrong(Message::new(MessageCode::OutputMismatch, why)),
            };
        }
        if let Checker::Rows { ordered, eps } = self {
            return match check_rows(expected, actual, *ordered, *eps) {
                None => CheckOutcome::Ok,
                Some(m) => CheckOutcome::Wrong(m),
            };
        }
        let ok = match self {
            Checker::Tokens | Checker::TokensFloat(_) | Checker::Lines | Checker::Rows { .. } => {
                unreachable!()
            }
            Checker::Exact => expected == actual,
            Checker::Unordered => unordered_eq(expected, actual),
            Checker::Float(eps) => float_eq(expected, actual, *eps),
            Checker::Custom => {
                return CheckOutcome::Wrong(Message::new(
                    MessageCode::CustomCheckerUnsupported,
                    "custom checker must run in the sandbox",
                ));
            }
        };
        if ok {
            CheckOutcome::Ok
        } else {
            CheckOutcome::Wrong(Message::new(
                MessageCode::OutputMismatch,
                format!("expected {expected}, got {actual}"),
            ))
        }
    }
}

// ---- Rows checkers (SQL problems, ADR 0031) ----------------------------------------------

fn sql_value_eq(a: &Value, b: &Value, eps: Option<f64>) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Number(x), Value::Number(y)) => {
            if let (Some(x), Some(y)) = (x.as_i64(), y.as_i64()) {
                return x == y || eps.is_some_and(|e| float_eq(a, b, e));
            }
            match (x.as_f64(), y.as_f64()) {
                (Some(fx), Some(fy)) => fx == fy || eps.is_some_and(|e| float_eq(a, b, e)),
                _ => false,
            }
        }
        _ => false,
    }
}

/// A total order on SQL values for sorting rows: NULL, then numbers, then text.
fn sql_value_cmp(a: &Value, b: &Value) -> std::cmp::Ordering {
    let rank = |v: &Value| match v {
        Value::Null => 0,
        Value::Number(_) => 1,
        _ => 2,
    };
    rank(a).cmp(&rank(b)).then_with(|| match (a, b) {
        (Value::Number(x), Value::Number(y)) => x
            .as_f64()
            .unwrap_or(0.0)
            .total_cmp(&y.as_f64().unwrap_or(0.0)),
        (Value::String(x), Value::String(y)) => x.cmp(y),
        _ => std::cmp::Ordering::Equal,
    })
}

fn row_cmp(a: &[Value], b: &[Value]) -> std::cmp::Ordering {
    a.iter()
        .zip(b)
        .map(|(x, y)| sql_value_cmp(x, y))
        .find(|o| o.is_ne())
        .unwrap_or(std::cmp::Ordering::Equal)
}

fn shown_row(row: &[Value]) -> String {
    let text = serde_json::to_string(row).unwrap_or_default();
    if text.chars().count() <= 120 {
        text
    } else {
        format!("{}…", text.chars().take(120).collect::<String>())
    }
}

/// `None` when the SQL results match, else why not: the columns first, then the count of
/// rows, then the first row that differs.
fn check_rows(
    expected: &Value,
    actual: &Value,
    ordered: bool,
    eps: Option<f64>,
) -> Option<Message> {
    let mismatch = |text: String| Some(Message::new(MessageCode::OutputMismatch, text));
    let parts = |v: &Value| -> Option<(Vec<String>, Vec<Vec<Value>>)> {
        let columns = v
            .get("columns")?
            .as_array()?
            .iter()
            .map(|c| c.as_str().map(str::to_string))
            .collect::<Option<Vec<_>>>()?;
        let rows = v
            .get("rows")?
            .as_array()?
            .iter()
            .map(|r| r.as_array().cloned())
            .collect::<Option<Vec<_>>>()?;
        Some((columns, rows))
    };
    let Some((ecols, mut erows)) = parts(expected) else {
        return mismatch("the expected result is malformed".into());
    };
    let Some((acols, mut arows)) = parts(actual) else {
        return mismatch("the result is malformed".into());
    };
    let same_columns = ecols.len() == acols.len()
        && ecols
            .iter()
            .zip(&acols)
            .all(|(e, a)| e.eq_ignore_ascii_case(a));
    if !same_columns {
        return Some(Message::new(
            MessageCode::SqlColumns,
            format!(
                "columns differ: expected {}, got {}",
                serde_json::to_string(&ecols).unwrap_or_default(),
                serde_json::to_string(&acols).unwrap_or_default()
            ),
        ));
    }
    if erows.len() != arows.len() {
        return mismatch(format!(
            "expected {} row{}, got {}",
            erows.len(),
            if erows.len() == 1 { "" } else { "s" },
            arows.len()
        ));
    }
    if !ordered {
        erows.sort_by(|a, b| row_cmp(a, b));
        arows.sort_by(|a, b| row_cmp(a, b));
    }
    for (k, (e, a)) in erows.iter().zip(&arows).enumerate() {
        let same = e.len() == a.len() && e.iter().zip(a).all(|(x, y)| sql_value_eq(x, y, eps));
        if !same {
            return mismatch(if ordered {
                format!(
                    "row {}: expected {}, got {}",
                    k + 1,
                    shown_row(e),
                    shown_row(a)
                )
            } else {
                format!(
                    "rows differ (in sorted order, row {}): expected {}, got {}",
                    k + 1,
                    shown_row(e),
                    shown_row(a)
                )
            });
        }
    }
    None
}

// ---- Text checkers (program problems, ADR 0028) -----------------------------------------

/// The longest token or line quoted in full in a mismatch message.
const SHOWN_CHARS: usize = 60;

fn shown(s: &str) -> String {
    if s.chars().count() <= SHOWN_CHARS {
        format!("`{s}`")
    } else {
        format!("`{}…`", s.chars().take(SHOWN_CHARS).collect::<String>())
    }
}

/// CRLF and a lone CR become LF, so Windows line endings never matter.
pub fn normalize_text(s: &str) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}

/// Tokens with their 1-based line and their 1-based position in that line.
fn tokens(s: &str) -> Vec<(usize, usize, &str)> {
    let mut out = Vec::new();
    for (l, line) in s.split('\n').enumerate() {
        for (k, tok) in line.split_ascii_whitespace().enumerate() {
            out.push((l + 1, k + 1, tok));
        }
    }
    out
}

/// Lines without trailing spaces and tabs, and without trailing empty lines.
fn lines(s: &str) -> Vec<&str> {
    let mut out: Vec<&str> = s
        .split('\n')
        .map(|l| l.trim_end_matches([' ', '\t']))
        .collect();
    while out.last().is_some_and(|l| l.is_empty()) {
        out.pop();
    }
    out
}

/// A token as a finite number (`nan` and `inf` stay words).
fn number(tok: &str) -> Option<f64> {
    tok.parse::<f64>().ok().filter(|x| x.is_finite())
}

impl Checker {
    /// `None` when `actual` matches `expected`, else why not, in words for the user.
    pub fn check_text(&self, expected: &str, actual: &str) -> Option<String> {
        use std::cmp::Ordering;
        let (expected, actual) = (normalize_text(expected), normalize_text(actual));
        if let Checker::Lines = self {
            let (e, a) = (lines(&expected), lines(&actual));
            for (i, (x, y)) in e.iter().zip(&a).enumerate() {
                if x != y {
                    return Some(format!(
                        "line {}: expected {}, got {}",
                        i + 1,
                        shown(x),
                        shown(y)
                    ));
                }
            }
            return match a.len().cmp(&e.len()) {
                Ordering::Less => Some(format!(
                    "output ended early: expected {} lines, got {}",
                    e.len(),
                    a.len()
                )),
                Ordering::Greater => Some(format!(
                    "extra output after the expected {} lines: line {} is {}",
                    e.len(),
                    e.len() + 1,
                    shown(a[e.len()])
                )),
                Ordering::Equal => None,
            };
        }
        let eps = match self {
            Checker::TokensFloat(eps) => Some(*eps),
            _ => None,
        };
        let (e, a) = (tokens(&expected), tokens(&actual));
        for ((line, k, x), (_, _, y)) in e.iter().zip(&a) {
            let same = x == y
                || eps.is_some_and(|eps| match (number(x), number(y)) {
                    (Some(x), Some(y)) => {
                        let diff = (x - y).abs();
                        diff <= eps || diff <= eps * x.abs().max(y.abs())
                    }
                    _ => false,
                });
            if !same {
                return Some(format!(
                    "line {line}, token {k}: expected {}, got {}",
                    shown(x),
                    shown(y)
                ));
            }
        }
        match a.len().cmp(&e.len()) {
            Ordering::Less => Some(format!(
                "output ended early: expected {} tokens, got {}",
                e.len(),
                a.len()
            )),
            Ordering::Greater => Some(format!(
                "extra output after the expected {} tokens: {}",
                e.len(),
                shown(a[e.len()].2)
            )),
            Ordering::Equal => None,
        }
    }
}

fn unordered_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Array(xs), Value::Array(ys)) if xs.len() == ys.len() => {
            let mut xs: Vec<String> = xs.iter().map(Value::to_string).collect();
            let mut ys: Vec<String> = ys.iter().map(Value::to_string).collect();
            xs.sort();
            ys.sort();
            xs == ys
        }
        _ => a == b,
    }
}

fn float_eq(a: &Value, b: &Value, eps: f64) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => match (x.as_f64(), y.as_f64()) {
            (Some(x), Some(y)) => {
                let diff = (x - y).abs();
                diff <= eps || diff <= eps * x.abs().max(y.abs())
            }
            _ => false,
        },
        (Value::Array(xs), Value::Array(ys)) => {
            xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| float_eq(x, y, eps))
        }
        (Value::Object(xs), Value::Object(ys)) => {
            xs.len() == ys.len()
                && xs
                    .iter()
                    .all(|(k, x)| ys.get(k).is_some_and(|y| float_eq(x, y, eps)))
        }
        _ => a == b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_all_forms() {
        assert_eq!("exact".parse(), Ok(Checker::Exact));
        assert_eq!("unordered".parse(), Ok(Checker::Unordered));
        assert_eq!("float:1e-6".parse(), Ok(Checker::Float(1e-6)));
        assert!("float:-1".parse::<Checker>().is_err());
        assert!("fuzzy".parse::<Checker>().is_err());
        assert_eq!("tokens".parse(), Ok(Checker::Tokens));
        assert_eq!(
            "rows-ordered".parse(),
            Ok(Checker::Rows {
                ordered: true,
                eps: None
            })
        );
        assert_eq!(
            "rows-unordered:0.01".parse(),
            Ok(Checker::Rows {
                ordered: false,
                eps: Some(0.01)
            })
        );
        assert!("rows-sorted".parse::<Checker>().is_err());
        assert!("rows-ordered:-1".parse::<Checker>().is_err());
        assert_eq!("lines".parse(), Ok(Checker::Lines));
        assert_eq!("tokens-float:1e-6".parse(), Ok(Checker::TokensFloat(1e-6)));
        assert!("tokens-float:0".parse::<Checker>().is_err());
        assert!("tokens-float:x".parse::<Checker>().is_err());
        assert!(Checker::Tokens.is_text() && !Checker::Exact.is_text());
    }

    fn text(c: &Checker, e: &str, a: &str) -> Option<String> {
        // check() and check_text() agree.
        let whole = c.check(&json!(e), &json!(a));
        assert_eq!(whole == CheckOutcome::Ok, c.check_text(e, a).is_none());
        c.check_text(e, a)
    }

    #[test]
    fn tokens_ignore_spacing_line_breaks_and_crlf() {
        let c = Checker::Tokens;
        assert_eq!(text(&c, "3 4\n5\n", "3   4 5"), None);
        assert_eq!(text(&c, "3 4\n5\n", "3\t4\r\n\r\n5\r\n\r\n"), None);
        assert_eq!(text(&c, "", ""), None);
        assert_eq!(text(&c, "", "\n \n"), None, "blank output is empty");
        assert_eq!(
            text(&c, "ok\n", "ok"),
            None,
            "the final newline does not matter"
        );
        assert_eq!(
            text(&c, "Año ñu\n", "Año  ñu"),
            None,
            "UTF-8 tokens compare as text"
        );
    }

    #[test]
    fn tokens_say_where_and_what() {
        let c = Checker::Tokens;
        assert_eq!(
            text(&c, "1 2\n3 7\n", "1 2\n3 8\n").unwrap(),
            "line 2, token 2: expected `7`, got `8`"
        );
        assert_eq!(
            text(&c, "1 2 3", "1 2").unwrap(),
            "output ended early: expected 3 tokens, got 2"
        );
        assert_eq!(
            text(&c, "1", "1 99").unwrap(),
            "extra output after the expected 1 tokens: `99`"
        );
        assert_eq!(
            text(&c, "1", "").unwrap(),
            "output ended early: expected 1 tokens, got 0"
        );
        // Case matters; a long token is cut in the message.
        assert!(text(&c, "YES", "yes").is_some());
        let long = "x".repeat(200);
        assert!(text(&c, "a", &long).unwrap().ends_with("…`"));
    }

    #[test]
    fn tokens_float_compares_numbers_only() {
        let c = Checker::TokensFloat(1e-6);
        assert_eq!(text(&c, "0.3333333 x\n", "0.33333331 x"), None);
        assert_eq!(
            text(&c, "1000000000", "1000000000.5"),
            None,
            "relative tolerance"
        );
        assert!(text(&c, "0.1", "0.2").is_some());
        // Words stay exact; nan and inf are words.
        assert!(text(&c, "x", "y").is_some());
        assert!(text(&c, "nan", "NaN").is_some());
        assert_eq!(text(&c, "inf", "inf"), None);
        assert!(text(&c, "1", "inf").is_some());
    }

    #[test]
    fn lines_ignore_trailing_spaces_and_final_newlines_only() {
        let c = Checker::Lines;
        assert_eq!(text(&c, "a b\nc\n", "a b  \r\nc\t\r\n\r\n"), None);
        assert_eq!(text(&c, "", ""), None);
        assert_eq!(text(&c, "x\n", "x"), None);
        assert_eq!(
            text(&c, "a b\nc\n", "a  b\nc\n").unwrap(),
            "line 1: expected `a b`, got `a  b`"
        );
        assert_eq!(
            text(&c, "a\n\nb\n", "a\nb\n").unwrap(),
            "line 2: expected ``, got `b`",
            "blank lines inside the output count"
        );
        assert_eq!(
            text(&c, "a\nb\n", "a\n").unwrap(),
            "output ended early: expected 2 lines, got 1"
        );
        assert_eq!(
            text(&c, "a\n", "a\nb\n").unwrap(),
            "extra output after the expected 1 lines: line 2 is `b`"
        );
        assert!(text(&c, " a", "a").is_some(), "leading spaces count");
        assert_eq!(text(&c, "año\n", "año\r\n"), None);
    }

    #[test]
    fn text_checkers_need_text() {
        assert!(matches!(
            Checker::Tokens.check(&json!("1"), &json!(1)),
            CheckOutcome::Wrong(_)
        ));
    }

    #[test]
    fn exact() {
        assert_eq!(
            Checker::Exact.check(&json!([0, 1]), &json!([0, 1])),
            CheckOutcome::Ok
        );
        assert!(matches!(
            Checker::Exact.check(&json!([0, 1]), &json!([1, 0])),
            CheckOutcome::Wrong(_)
        ));
    }

    #[test]
    fn mismatch_has_a_stable_code() {
        let CheckOutcome::Wrong(m) = Checker::Exact.check(&json!(1), &json!(2)) else {
            panic!("expected a mismatch")
        };
        assert_eq!(m.code, MessageCode::OutputMismatch);
        assert_eq!(m.text, "expected 1, got 2");
    }

    #[test]
    fn unordered_ignores_order_but_not_multiplicity() {
        let c = Checker::Unordered;
        assert_eq!(c.check(&json!([0, 1]), &json!([1, 0])), CheckOutcome::Ok);
        assert!(matches!(
            c.check(&json!([1, 1, 2]), &json!([1, 2, 2])),
            CheckOutcome::Wrong(_)
        ));
    }

    #[test]
    fn float_tolerance_is_recursive() {
        let c = Checker::Float(1e-6);
        assert_eq!(
            c.check(&json!([0.1, 2.0]), &json!([0.1000000001, 2.0])),
            CheckOutcome::Ok
        );
        assert_eq!(
            c.check(&json!({"x": 1e9}), &json!({"x": 1.0000000001e9})),
            CheckOutcome::Ok
        );
        assert!(matches!(
            c.check(&json!(0.1), &json!(0.2)),
            CheckOutcome::Wrong(_)
        ));
    }

    #[test]
    fn rows_checkers_compare_columns_then_rows() {
        use serde_json::json;
        let un: Checker = "rows-unordered".parse().unwrap();
        let ord: Checker = "rows-ordered".parse().unwrap();
        let exp =
            json!({"columns": ["name", "total"], "rows": [["ana", 2], ["bo", 1.5], [null, 0]]});
        // Order ignored; 2 equals 2.0; column names case-insensitive.
        let shuffled =
            json!({"columns": ["NAME", "Total"], "rows": [[null, 0], ["ana", 2.0], ["bo", 1.5]]});
        assert_eq!(un.check(&exp, &shuffled), CheckOutcome::Ok);
        assert!(matches!(ord.check(&exp, &shuffled), CheckOutcome::Wrong(_)));
        // A wrong column name is reported first, with both lists.
        let renamed = json!({"columns": ["name", "sum"], "rows": [["x", 9]]});
        match un.check(&exp, &renamed) {
            CheckOutcome::Wrong(m) => {
                assert_eq!(m.code, MessageCode::SqlColumns);
                assert!(
                    m.text.contains("[\"name\",\"total\"]") && m.text.contains("sum"),
                    "{}",
                    m.text
                );
            }
            other => panic!("{other:?}"),
        }
        // Row count, then the first differing row; NULL only equals NULL; text is exact.
        let fewer = json!({"columns": ["name", "total"], "rows": [["ana", 2]]});
        assert!(
            matches!(un.check(&exp, &fewer), CheckOutcome::Wrong(m) if m.text.contains("expected 3 rows, got 1"))
        );
        let null_vs_zero =
            json!({"columns": ["name", "total"], "rows": [["ana", 2], ["bo", 1.5], [null, null]]});
        assert!(matches!(
            un.check(&exp, &null_vs_zero),
            CheckOutcome::Wrong(_)
        ));
        let case =
            json!({"columns": ["name", "total"], "rows": [["Ana", 2], ["bo", 1.5], [null, 0]]});
        assert!(matches!(un.check(&exp, &case), CheckOutcome::Wrong(_)));
        let text_number =
            json!({"columns": ["name", "total"], "rows": [["ana", "2"], ["bo", 1.5], [null, 0]]});
        assert!(matches!(
            un.check(&exp, &text_number),
            CheckOutcome::Wrong(_)
        ));
        // A multiset: duplicates count.
        let dup_e = json!({"columns": ["a"], "rows": [[1], [1], [2]]});
        let dup_a = json!({"columns": ["a"], "rows": [[1], [2], [2]]});
        assert!(matches!(un.check(&dup_e, &dup_a), CheckOutcome::Wrong(_)));
        // A tolerance when asked for.
        let eps: Checker = "rows-ordered:0.001".parse().unwrap();
        let near = json!({"columns": ["a"], "rows": [[0.3333]]});
        let third = json!({"columns": ["a"], "rows": [[0.33333333]]});
        assert_eq!(eps.check(&near, &third), CheckOutcome::Ok);
        assert!(matches!(ord.check(&near, &third), CheckOutcome::Wrong(_)));
    }
}
