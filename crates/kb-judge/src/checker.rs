//! Built-in checkers. Values are compared as JSON so every language's harness
//! only has to serialize its return value.

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
}

impl FromStr for Checker {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exact" => Ok(Checker::Exact),
            "unordered" => Ok(Checker::Unordered),
            "custom" => Ok(Checker::Custom),
            _ => {
                if let Some(eps) = s.strip_prefix("float:") {
                    let eps: f64 = eps.parse().map_err(|_| format!("bad tolerance in {s:?}"))?;
                    if eps > 0.0 && eps.is_finite() {
                        return Ok(Checker::Float(eps));
                    }
                }
                Err(format!(
                    "unknown checker {s:?} (expected exact | unordered | float:<eps> | custom)"
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
    /// Compare `actual` against `expected`. `Custom` always returns an error message
    /// because it must be executed by the runner.
    pub fn check(&self, expected: &Value, actual: &Value) -> CheckOutcome {
        let ok = match self {
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
