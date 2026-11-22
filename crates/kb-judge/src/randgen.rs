//! Small random inputs from a signature alone (expansion M8, ADR 0036): the stress lab's
//! fallback when gen.py cannot run (no Python runtime). Pure and seeded: the same seed
//! gives the same inputs. Values are deliberately small (arrays of 0-8, ints in −20..20,
//! short strings over a three-letter alphabet), where most bugs already show. They are
//! not checked against the problem's own constraints (validate.py needs Python); the UI
//! says so.

use serde_json::{Map, Value, json};

use crate::signature::{Signature, Type};

/// xorshift64*: tiny, deterministic, good enough for test data.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        // Never zero (xorshift's fixed point); mix the seed a little.
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform in `lo..=hi`.
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        let span = (hi - lo + 1) as u64;
        lo + (self.next_u64() % span) as i64
    }
}

/// Largest array length.
pub const MAX_LEN: i64 = 8;
/// Ints are in `-MAX_INT..=MAX_INT`.
pub const MAX_INT: i64 = 20;

fn int(rng: &mut Rng) -> i64 {
    // Half the time a very small value, so equal values and zero are common.
    if rng.range(0, 1) == 0 {
        rng.range(-3, 3)
    } else {
        rng.range(-MAX_INT, MAX_INT)
    }
}

fn tree(rng: &mut Rng) -> Value {
    // Grow a random binary tree of 0-7 nodes, then write its level order with nulls.
    let n = rng.range(0, 7) as usize;
    if n == 0 {
        return json!([]);
    }
    let mut kids: Vec<[Option<usize>; 2]> = vec![[None, None]];
    for k in 1..n {
        loop {
            let parent = rng.range(0, (kids.len() - 1) as i64) as usize;
            let side = rng.range(0, 1) as usize;
            if kids[parent][side].is_none() {
                kids[parent][side] = Some(k);
                kids.push([None, None]);
                break;
            }
        }
    }
    let mut out = Vec::new();
    let mut level = vec![Some(0usize)];
    while level.iter().any(Option::is_some) {
        let mut next = Vec::new();
        for slot in level {
            match slot {
                Some(node) => {
                    out.push(json!(int(rng)));
                    next.extend(kids[node]);
                }
                None => out.push(Value::Null),
            }
        }
        level = next;
    }
    while out.last() == Some(&Value::Null) {
        out.pop();
    }
    Value::Array(out)
}

/// One random value of type `ty`; `depth` shortens nested arrays.
pub fn value(ty: &Type, rng: &mut Rng, depth: u32) -> Value {
    match ty {
        Type::Int => json!(int(rng)),
        Type::Float => json!(int(rng) as f64 / 2.0),
        Type::Bool => json!(rng.range(0, 1) == 1),
        Type::String => {
            let len = rng.range(0, 6);
            let s: String = (0..len)
                .map(|_| (b'a' + rng.range(0, 2) as u8) as char)
                .collect();
            json!(s)
        }
        Type::ListNode => {
            let len = rng.range(0, MAX_LEN);
            Value::Array((0..len).map(|_| json!(int(rng))).collect())
        }
        Type::TreeNode => tree(rng),
        Type::Array(inner) => {
            let max = if depth == 0 { MAX_LEN } else { 4 };
            let len = rng.range(0, max);
            Value::Array((0..len).map(|_| value(inner, rng, depth + 1)).collect())
        }
    }
}

/// `count` inputs for `sig`, from `seed`, that pass the signature's type checks.
pub fn inputs(sig: &Signature, seed: u64, count: usize) -> Vec<Map<String, Value>> {
    let mut rng = Rng::new(seed);
    let mut out = Vec::with_capacity(count);
    // A few extra tries for inputs the type check refuses (it never should).
    for _ in 0..count * 4 {
        if out.len() == count {
            break;
        }
        let input: Map<String, Value> = sig
            .params
            .iter()
            .map(|p| (p.name.clone(), value(&p.ty, &mut rng, 0)))
            .collect();
        if crate::problem::check_input(&input, sig).is_ok() {
            out.push(input);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signature::Param;

    fn sig(params: &[(&str, &str)]) -> Signature {
        Signature {
            name: "f".into(),
            params: params
                .iter()
                .map(|(n, t)| Param {
                    name: (*n).into(),
                    ty: t.parse().unwrap(),
                })
                .collect(),
            returns: Type::Int,
        }
    }

    #[test]
    fn the_same_seed_gives_the_same_inputs() {
        let s = sig(&[("nums", "int[]"), ("k", "int")]);
        assert_eq!(inputs(&s, 7, 16), inputs(&s, 7, 16));
        assert_ne!(inputs(&s, 7, 16), inputs(&s, 8, 16));
    }

    #[test]
    fn values_are_small_and_typed() {
        let s = sig(&[
            ("nums", "int[]"),
            ("grid", "int[][]"),
            ("word", "string"),
            ("root", "TreeNode"),
            ("head", "ListNode"),
            ("on", "bool"),
            ("x", "float"),
        ]);
        let all = inputs(&s, 1, 200);
        assert_eq!(all.len(), 200);
        for i in &all {
            let nums = i["nums"].as_array().unwrap();
            assert!(nums.len() as i64 <= MAX_LEN);
            assert!(nums.iter().all(|v| v.as_i64().unwrap().abs() <= MAX_INT));
            assert!(
                i["grid"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|r| r.as_array().unwrap().len() <= 4)
            );
            assert!(
                i["word"]
                    .as_str()
                    .unwrap()
                    .chars()
                    .all(|c| "abc".contains(c))
            );
            let tree = i["root"].as_array().unwrap();
            assert!(
                tree.first().is_none_or(|v| !v.is_null()),
                "a tree's root is never null"
            );
            assert!(
                tree.last().is_none_or(|v| !v.is_null()),
                "trailing nulls are trimmed"
            );
        }
        // Empty arrays and repeated values both show up.
        assert!(all.iter().any(|i| i["nums"].as_array().unwrap().is_empty()));
        assert!(all.iter().any(|i| {
            let n: Vec<i64> = i["nums"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_i64().unwrap())
                .collect();
            (1..n.len()).any(|k| n[..k].contains(&n[k]))
        }));
    }
}
