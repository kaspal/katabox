//! The failing-test shrinker's candidate moves (ADR 0030): given a failing input, smaller
//! inputs of the same types, biggest cuts first. Pure; the loop that judges them (validate.py,
//! the reference, the user's code) is in kb-app.
//!
//! Every candidate is strictly smaller by [`size`], so a shrink always ends.

use serde_json::{Map, Number, Value};

use crate::signature::{Signature, Type};

/// How big an input is: what shrinking minimises. Containers count their items and the
/// items' own sizes; strings their characters; numbers their digits; `true` 1.
pub fn size(v: &Value) -> u64 {
    match v {
        Value::Null => 0,
        Value::Bool(b) => u64::from(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                // 0 is 0; then the number of decimal digits, negative a little bigger.
                let digits = if i == 0 {
                    0
                } else {
                    u64::from(i.unsigned_abs().ilog10()) + 1
                };
                digits + u64::from(i < 0)
            } else if let Some(f) = n.as_f64() {
                if f == 0.0 {
                    0
                } else {
                    (f.abs().log10().max(0.0) as u64) + 2 + u64::from(f < 0.0)
                }
            } else {
                1
            }
        }
        Value::String(s) => s.chars().count() as u64,
        Value::Array(a) => a.len() as u64 + a.iter().map(size).sum::<u64>(),
        Value::Object(o) => o.values().map(size).sum(),
    }
}

/// The size of a whole input (every parameter).
pub fn input_size(input: &Map<String, Value>) -> u64 {
    input.values().map(size).sum()
}

fn int(i: i64) -> Value {
    Value::Number(Number::from(i))
}

/// Smaller numbers, nearest to zero first.
fn shrink_int(i: i64) -> Vec<Value> {
    let mut out = Vec::new();
    if i == 0 {
        return out;
    }
    out.push(int(0));
    if i.abs() > 1 {
        out.push(int(i.signum()));
    }
    if i < 0 {
        out.push(int(-i)); // same digits but not negative: smaller by one
    }
    let half = i / 2;
    if half != 0 && half != i.signum() {
        out.push(int(half));
    }
    out
}

fn shrink_float(f: f64) -> Vec<Value> {
    let mut out = Vec::new();
    if f == 0.0 {
        return out;
    }
    out.push(Value::from(0.0));
    if f.fract() != 0.0 {
        out.push(Value::from(f.trunc()));
    }
    if f.abs() > 1.0 {
        out.push(Value::from((f / 2.0).trunc()));
    }
    out
}

fn shrink_string(s: &str) -> Vec<Value> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut out = Vec::new();
    if n == 0 {
        return out;
    }
    let take = |a: usize, b: usize| -> Value { Value::String(chars[a..b].iter().collect()) };
    if n > 1 {
        out.push(take(0, n / 2));
        out.push(take(n / 2, n));
    }
    // Drop one character (a few positions, so a long string stays affordable).
    for k in positions(n) {
        let mut c = chars.clone();
        c.remove(k);
        out.push(Value::String(c.into_iter().collect()));
    }
    out
}

/// Up to 8 indices spread over 0..n.
fn positions(n: usize) -> Vec<usize> {
    if n <= 8 {
        return (0..n).collect();
    }
    let mut v: Vec<usize> = (0..8).map(|k| k * (n - 1) / 7).collect();
    v.dedup();
    v
}

/// Smaller arrays of `elem`: halves, then quarters, then single items removed, then (for
/// short arrays) each element shrunk.
fn shrink_array(items: &[Value], elem: &Type, keep_tail_only: bool) -> Vec<Value> {
    let n = items.len();
    let mut out = Vec::new();
    if n == 0 {
        return out;
    }
    let arr = |v: Vec<Value>| Value::Array(v);
    if keep_tail_only {
        // A tree in level order: only cutting from the end keeps it a valid tree.
        for keep in [n / 2, n - n / 4, n - 1] {
            if keep < n && keep > 0 {
                out.push(arr(items[..keep].to_vec()));
            }
        }
        if n == 1 {
            out.push(arr(vec![]));
        }
    } else {
        out.push(arr(vec![]));
        if n > 1 {
            out.push(arr(items[..n / 2].to_vec()));
            out.push(arr(items[n / 2..].to_vec()));
        }
        if n > 3 {
            let q = n / 4;
            for k in 0..4 {
                let (a, b) = (k * q, if k == 3 { n } else { (k + 1) * q });
                let mut v = items[..a].to_vec();
                v.extend_from_slice(&items[b..]);
                out.push(arr(v));
            }
        }
        for k in positions(n) {
            let mut v = items.to_vec();
            v.remove(k);
            out.push(arr(v));
        }
    }
    if n <= 8 {
        for k in 0..n {
            for smaller in shrink_value(&items[k], elem) {
                let mut v = items.to_vec();
                v[k] = smaller;
                out.push(arr(v));
            }
        }
    }
    out
}

/// Smaller values of type `ty`, biggest cuts first.
pub fn shrink_value(v: &Value, ty: &Type) -> Vec<Value> {
    match (ty, v) {
        (Type::Int, Value::Number(n)) => n.as_i64().map(shrink_int).unwrap_or_default(),
        (Type::Float, Value::Number(n)) => n.as_f64().map(shrink_float).unwrap_or_default(),
        (Type::String, Value::String(s)) => shrink_string(s),
        (Type::Bool, Value::Bool(true)) => vec![Value::Bool(false)],
        // A list is its values (ADR 0025).
        (Type::ListNode, Value::Array(a)) => shrink_array(a, &Type::Int, false),
        // A tree is its level order with nulls; values shrink, the shape only from the end.
        // Trailing nulls are trimmed, as in the canonical form.
        (Type::TreeNode, Value::Array(a)) => shrink_array(a, &Type::Int, true)
            .into_iter()
            .map(|c| match c {
                Value::Array(mut x) => {
                    while x.last().is_some_and(Value::is_null) {
                        x.pop();
                    }
                    Value::Array(x)
                }
                other => other,
            })
            .filter(|c| {
                c.as_array()
                    .is_none_or(|x| x.first().is_none_or(|r| !r.is_null()))
            })
            .filter(|c| c != v)
            .collect(),
        (Type::Array(inner), Value::Array(a)) => shrink_array(a, inner, false),
        _ => Vec::new(),
    }
}

/// Smaller inputs: one parameter changed at a time, biggest cuts first, each strictly
/// smaller than `input`, at most `max`.
pub fn candidates(
    input: &Map<String, Value>,
    sig: &Signature,
    max: usize,
) -> Vec<Map<String, Value>> {
    let base = input_size(input);
    let mut out: Vec<(u64, Map<String, Value>)> = Vec::new();
    for p in &sig.params {
        let Some(v) = input.get(&p.name) else {
            continue;
        };
        for smaller in shrink_value(v, &p.ty) {
            let mut c = input.clone();
            c.insert(p.name.clone(), smaller);
            let s = input_size(&c);
            if s < base && !out.iter().any(|(_, o)| *o == c) {
                out.push((s, c));
            }
        }
    }
    // Biggest cut first; ties keep the order above (parameters, then moves).
    out.sort_by_key(|(s, _)| *s);
    out.into_iter().take(max).map(|(_, c)| c).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sig(params: &str) -> Signature {
        let params = params
            .split(',')
            .map(|p| {
                let (n, t) = p.split_once(':').unwrap();
                crate::signature::Param {
                    name: n.trim().into(),
                    ty: t.trim().parse().unwrap(),
                }
            })
            .collect();
        Signature {
            name: "f".into(),
            params,
            returns: Type::Int,
        }
    }

    fn input(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn every_candidate_is_smaller_and_keeps_the_types() {
        let s = sig(
            "nums: int[], target: int, word: string, grid: int[][], head: ListNode, root: TreeNode, on: bool",
        );
        let i = input(json!({
            "nums": [5, -12, 300, 7, 7, 1, 0, 42, 9],
            "target": -15,
            "word": "shrinking",
            "grid": [[1, 2], [3, 4]],
            "head": [3, 1, 2],
            "root": [4, 2, 6, null, 3],
            "on": true
        }));
        let cands = candidates(&i, &s, 1000);
        assert!(!cands.is_empty());
        for c in &cands {
            assert!(input_size(c) < input_size(&i), "{c:?}");
            assert!(crate::problem::check_input(c, &s).is_ok(), "{c:?}");
            // A tree never starts with a null root once cut.
            if let Some(r) = c["root"].as_array() {
                assert!(r.first().is_none_or(|x| !x.is_null()));
            }
        }
        // The biggest cuts come first: the first candidate empties an array.
        assert!(cands[0].values().any(|v| v == &json!([])), "{:?}", cands[0]);
    }

    #[test]
    fn shrinking_repeatedly_reaches_a_minimum() {
        // A "bug" that fails whenever the array holds a 7: the minimum is [7].
        let s = sig("nums: int[]");
        let mut cur = input(json!({"nums": [3, 9, 7, 12, 40, 7, 1, 5]}));
        let fails = |c: &Map<String, Value>| c["nums"].as_array().unwrap().contains(&json!(7));
        for _ in 0..100 {
            match candidates(&cur, &s, 16).into_iter().find(|c| fails(c)) {
                Some(c) => cur = c,
                None => break,
            }
        }
        assert_eq!(cur["nums"], json!([7]));
    }

    #[test]
    fn sizes() {
        assert_eq!(size(&json!(0)), 0);
        assert_eq!(size(&json!(9)), 1);
        assert_eq!(size(&json!(-10)), 3);
        assert_eq!(size(&json!("abc")), 3);
        assert_eq!(size(&json!([1, 22])), 2 + 1 + 2);
        assert!(shrink_value(&json!(0), &Type::Int).is_empty());
        assert!(shrink_value(&json!(false), &Type::Bool).is_empty());
    }
}
