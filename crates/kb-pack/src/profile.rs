//! Complexity profiler data (ADR 0024).
//!
//! A problem with `profile = true` lists its `profile_sizes` in problem.toml; its gen.py
//! answers `generate(seed, n)` for an int n. `kb-pack profile` (sandboxed) writes, and
//! commits like the hidden tests:
//!
//! - `profile/inputs/<n>.json`: the input for each size (seed 1);
//! - `profile/reference.json`: `{sizes, machine_factor, times_us: {lang: [...]}}`, the
//!   reference solutions' median per-call times, scaled to the reference machine.
//!
//! `kb-pack validate` checks the files against the sizes and the profile budget;
//! `validate --run` regenerates the inputs and checks they match (gen.py honours n).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use kb_core::Language;
use kb_judge::profile::SizedInput;
use kb_judge::{JudgeEnv, Problem, ScriptLimits};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The committed profile inputs of one problem may take at most this much compressed.
pub const PROFILE_BUDGET_BYTES: u64 = 512 * 1024;
/// How many sizes a profile may list.
pub const SIZES_MIN: usize = 4;
pub const SIZES_MAX: usize = 8;
/// The seed of every profile input.
pub const SEED: u64 = 1;

/// `profile/reference.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub sizes: Vec<u32>,
    /// The machine that measured, relative to the reference machine (1.0).
    pub machine_factor: f64,
    /// Per language id: the median per-call time at each size, on the reference machine.
    pub times_us: BTreeMap<String, Vec<u32>>,
}

pub fn input_path(dir: &Path, n: u32) -> PathBuf {
    dir.join("profile").join("inputs").join(format!("{n}.json"))
}

pub fn reference_path(dir: &Path) -> PathBuf {
    dir.join("profile").join("reference.json")
}

/// The rules for `profile_sizes` alone.
pub fn sizes_error(sizes: &[u32]) -> Option<String> {
    if !(SIZES_MIN..=SIZES_MAX).contains(&sizes.len()) {
        return Some(format!(
            "profile_sizes lists {SIZES_MIN} to {SIZES_MAX} sizes"
        ));
    }
    if sizes[0] == 0 || sizes.windows(2).any(|w| w[0] >= w[1]) {
        return Some("profile_sizes are positive and strictly increasing".into());
    }
    None
}

/// The committed profile files of a problem: present, matching the sizes, in budget.
pub fn files_error(dir: &Path, sizes: &[u32]) -> Option<String> {
    let mut compressed = 0u64;
    for &n in sizes {
        let path = input_path(dir, n);
        let Ok(text) = fs::read_to_string(&path) else {
            return Some(format!("missing {} (run kb-pack profile)", path.display()));
        };
        let value = match serde_json::from_str::<Value>(&text) {
            Ok(v @ Value::Object(_)) => v,
            _ => return Some(format!("{} is not a JSON object", path.display())),
        };
        compressed += kb_store::blob::compress_json(&value)
            .map(|b| b.len() as u64)
            .unwrap_or(0);
    }
    let listed: Vec<u32> = fs::read_dir(dir.join("profile").join("inputs"))
        .map(|rd| {
            rd.flatten()
                .filter_map(|e| e.path().file_stem()?.to_str()?.parse::<u32>().ok())
                .collect()
        })
        .unwrap_or_default();
    if let Some(extra) = listed.iter().find(|n| !sizes.contains(n)) {
        return Some(format!(
            "profile/inputs/{extra}.json is not in profile_sizes"
        ));
    }
    if compressed > PROFILE_BUDGET_BYTES {
        return Some(format!(
            "profile inputs take {} KB compressed, over the {} KB budget",
            compressed / 1024,
            PROFILE_BUDGET_BYTES / 1024
        ));
    }
    match read_reference(dir) {
        Err(e) => Some(e),
        Ok(r) if r.sizes != sizes => Some("profile/reference.json is for other sizes".into()),
        Ok(r) => {
            kb_judge::languages::SUPPORTED
                .into_iter()
                .find_map(|l| match r.times_us.get(l.id()) {
                    Some(t) if t.len() == sizes.len() => None,
                    _ => Some(format!(
                        "profile/reference.json has no times for {}",
                        l.id()
                    )),
                })
        }
    }
}

pub fn read_reference(dir: &Path) -> Result<Reference, String> {
    let path = reference_path(dir);
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// The committed input for each size.
pub fn read_inputs(dir: &Path, sizes: &[u32]) -> Result<Vec<SizedInput>, String> {
    sizes
        .iter()
        .map(|&n| {
            let path = input_path(dir, n);
            let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            match serde_json::from_str(&text) {
                Ok(Value::Object(m)) => Ok((n, m)),
                _ => Err(format!("{} is not a JSON object", path.display())),
            }
        })
        .collect()
}

/// Generate the inputs with gen.py and check them with validate.py (sandboxed).
fn generate(dir: &Path, env: &JudgeEnv, sizes: &[u32]) -> Result<Vec<Map<String, Value>>, String> {
    let generator = fs::read_to_string(dir.join("gen.py")).map_err(|e| format!("gen.py: {e}"))?;
    let inputs = kb_judge::generate_sized(env, &generator, SEED, sizes, ScriptLimits::default())
        .map_err(|e| e.to_string())?;
    if let Ok(v) = fs::read_to_string(dir.join("validate.py")) {
        let refs: Vec<&Map<String, Value>> = inputs.iter().collect();
        let answers = kb_judge::validate_inputs(env, &v, &refs, ScriptLimits::default())
            .map_err(|e| e.to_string())?;
        if let Some((i, msg)) = answers
            .iter()
            .enumerate()
            .find_map(|(i, a)| a.as_ref().map(|m| (i, m)))
        {
            return Err(format!(
                "validate.py rejects the profile input for n = {}: {msg}",
                sizes[i]
            ));
        }
    }
    let lengths: Vec<usize> = inputs
        .iter()
        .map(|m| Value::Object(m.clone()).to_string().len())
        .collect();
    // Inputs never shrink with n, and the largest is bigger than the smallest (a single
    // number like `{"n": 25}` grows only with its digits).
    if lengths.windows(2).any(|w| w[0] > w[1]) || lengths.first() >= lengths.last() {
        return Err(format!(
            "gen.py does not honour n: the profile inputs do not grow with n (sizes {sizes:?}, {lengths:?} bytes)"
        ));
    }
    Ok(inputs)
}

/// `validate --run`: gen.py still produces exactly the committed inputs.
pub fn check_regenerates(dir: &Path, env: &JudgeEnv, sizes: &[u32]) -> Result<String, String> {
    let fresh = generate(dir, env, sizes)?;
    let committed = read_inputs(dir, sizes)?;
    for ((n, old), new) in committed.iter().zip(&fresh) {
        if old != new {
            return Err(format!(
                "gen.py's input for n = {n} differs from profile/inputs/{n}.json"
            ));
        }
    }
    Ok(format!(
        "{} sizes regenerate from gen.py and pass validate.py",
        sizes.len()
    ))
}

/// `kb-pack profile DIR`: write the inputs and the reference times.
pub fn write_profile(dir: &Path, env: &JudgeEnv, sizes: &[u32]) -> Result<Vec<String>, String> {
    let problem = Problem::load(dir).map_err(|e| e.to_string())?;
    let inputs = generate(dir, env, sizes)?;
    let folder = dir.join("profile").join("inputs");
    if folder.exists() {
        fs::remove_dir_all(&folder).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
    let mut lines = Vec::new();
    for (&n, input) in sizes.iter().zip(&inputs) {
        let text =
            serde_json::to_string(&Value::Object(input.clone())).map_err(|e| e.to_string())?;
        fs::write(input_path(dir, n), text + "\n").map_err(|e| e.to_string())?;
    }
    let factor = kb_judge::calibration::calibrate(env)
        .map(|c| c.factor)
        .unwrap_or(1.0);
    let sized: Vec<SizedInput> = sizes.iter().copied().zip(inputs).collect();
    let mut times = BTreeMap::new();
    for lang in kb_judge::languages::SUPPORTED {
        let path = dir
            .join("solutions")
            .join(format!("ref.{}", lang.extension()));
        let source = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let run = kb_judge::profile::profile(
            &problem,
            lang,
            &source,
            &sized,
            env,
            Duration::from_secs(300),
        )
        .map_err(|e| e.to_string())?;
        if let Some(stop) = run.stopped {
            return Err(format!(
                "ref.{} did not finish every size: {stop:?}",
                lang.extension()
            ));
        }
        let us: Vec<u32> = run
            .points
            .iter()
            .map(|p| (f64::from(p.median_us) / factor).round().max(1.0) as u32)
            .collect();
        lines.push(format!("{:<10} {us:?} µs", lang.id()));
        times.insert(lang.id().to_string(), us);
    }
    let reference = Reference {
        sizes: sizes.to_vec(),
        machine_factor: factor,
        times_us: times,
    };
    let text = serde_json::to_string_pretty(&reference).map_err(|e| e.to_string())?;
    fs::write(reference_path(dir), text + "\n").map_err(|e| e.to_string())?;
    lines.insert(0, format!("machine factor {factor:.2}; sizes {sizes:?}"));
    Ok(lines)
}

/// `kb-pack profile DIR`: the sizes come from problem.toml.
pub fn profile_problem(dir: &Path, env: &JudgeEnv) -> Result<Vec<String>, String> {
    let meta = crate::validate::read_problem_toml(dir)?;
    let Some(sizes) = meta.profile_sizes.filter(|_| meta.profile) else {
        return Err("problem.toml has no profile = true with profile_sizes".into());
    };
    if let Some(e) = sizes_error(&sizes) {
        return Err(e);
    }
    write_profile(dir, env, &sizes)
}

/// The languages a profile reference covers (every supported one).
pub fn languages() -> [Language; 4] {
    kb_judge::languages::SUPPORTED
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_rules() {
        assert!(sizes_error(&[100, 300, 1000, 3000]).is_none());
        assert!(sizes_error(&[100, 300, 1000]).is_some());
        assert!(sizes_error(&[100, 300, 300, 1000]).is_some());
        assert!(sizes_error(&[0, 3, 10, 30]).is_some());
        assert!(sizes_error(&[1, 2, 3, 4, 5, 6, 7, 8, 9]).is_some());
    }
}
