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
                Err(format!(
                    "unknown checker {s:?} (expected exact | unordered | float:<eps> | custom | \
                     tokens | tokens-float:<eps> | lines)"
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
        let ok = match self {
            Checker::Tokens | Checker::TokensFloat(_) | Checker::Lines => unreachable!(),
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
}
