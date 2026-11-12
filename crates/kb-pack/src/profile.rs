//! Complexity profiler data (ADR 0024).
//!
//! A problem with `profile = true` lists its `profile_sizes` in problem.toml; its gen.py
//! answers `generate(seed, n)` for an int n. `kb-pack profile` (sandboxed) writes:
//!
//! - `profile/reference.json` (**committed**): `{sizes, seed, sha256, machine_factor,
//!   times_us: {lang: [...]}}`: the SHA-256 of each size's input and the reference
//!   solutions' median per-call times, scaled to the reference machine;
//! - `profile/inputs/<n>.json` (**not committed**, gitignored): the inputs themselves, a
//!   cache that `kb-pack profile --inputs` and `validate --run` regenerate from gen.py
//!   and verify against the hashes.
//!
//! Unlike tests (ADR 0018), profile inputs are generated, not reviewed by hand, so only
//! their hashes live in git. `kb-pack build` packs a problem's profile when its cache is
//! complete and matches; a stale cache fails the build, a missing one leaves the problem
//! without profile data (with a warning), so builds never need the toolchains.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use kb_core::Language;
use kb_judge::profile::SizedInput;
use kb_judge::{JudgeEnv, Problem, ScriptLimits};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

/// The profile inputs of one problem may take at most this much compressed in content.db.
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
    /// gen.py's seed for every size.
    pub seed: u64,
    /// The SHA-256 (hex) of each size's input as compact JSON, in size order.
    pub sha256: Vec<String>,
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

/// An input as hashed and cached: compact JSON.
pub fn input_text(input: &Map<String, Value>) -> String {
    Value::Object(input.clone()).to_string()
}

/// SHA-256 of `text`, lowercase hex.
pub fn sha256_hex(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// The inputs over the profile budget (compressed as content.db stores them)?
pub fn budget_error(inputs: &[Map<String, Value>]) -> Option<String> {
    let compressed: u64 = inputs
        .iter()
        .map(|m| {
            kb_store::blob::compress_json(&Value::Object(m.clone()))
                .map(|b| b.len() as u64)
                .unwrap_or(0)
        })
        .sum();
    (compressed > PROFILE_BUDGET_BYTES).then(|| {
        format!(
            "profile inputs take {} KB compressed, over the {} KB budget",
            compressed / 1024,
            PROFILE_BUDGET_BYTES / 1024
        )
    })
}

/// The committed reference (sizes, seed, hashes, times) and any cached inputs: a cached
/// input must match its hash. A missing cache is fine (the build skips the profile).
pub fn files_error(dir: &Path, sizes: &[u32]) -> Option<String> {
    let reference = match read_reference(dir) {
        Ok(r) => r,
        Err(e) => return Some(e),
    };
    if reference.sizes != sizes {
        return Some("profile/reference.json is for other sizes".into());
    }
    if reference.seed != SEED {
        return Some(format!("profile/reference.json: the seed must be {SEED}"));
    }
    let hex = |h: &String| {
        h.len() == 64
            && h.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    };
    if reference.sha256.len() != sizes.len() || !reference.sha256.iter().all(hex) {
        return Some("profile/reference.json needs one SHA-256 (hex) per size".into());
    }
    if let Some(l) = kb_judge::languages::SUPPORTED.into_iter().find(|l| {
        reference
            .times_us
            .get(l.id())
            .is_none_or(|t| t.len() != sizes.len())
    }) {
        return Some(format!(
            "profile/reference.json has no times for {}",
            l.id()
        ));
    }
    if let Err(e) = cached_inputs(dir, &reference) {
        return Some(e);
    }
    let listed: Vec<u32> = fs::read_dir(dir.join("profile").join("inputs"))
        .map(|rd| {
            rd.flatten()
                .filter_map(|e| e.path().file_stem()?.to_str()?.parse::<u32>().ok())
                .collect()
        })
        .unwrap_or_default();
    listed
        .iter()
        .find(|n| !sizes.contains(n))
        .map(|extra| format!("profile/inputs/{extra}.json is not in profile_sizes"))
}

/// The cached inputs, verified against the reference's hashes: `None` when any is
/// missing, an error when one does not match (a stale cache: run `kb-pack profile
/// --inputs`).
pub fn cached_inputs(dir: &Path, reference: &Reference) -> Result<Option<Vec<SizedInput>>, String> {
    let mut out = Vec::new();
    for (&n, want) in reference.sizes.iter().zip(&reference.sha256) {
        let path = input_path(dir, n);
        let Ok(text) = fs::read_to_string(&path) else {
            return Ok(None);
        };
        let input = match serde_json::from_str(&text) {
            Ok(Value::Object(m)) => m,
            _ => return Err(format!("{} is not a JSON object", path.display())),
        };
        if sha256_hex(&input_text(&input)) != *want {
            return Err(format!(
                "profile/inputs/{n}.json does not match its SHA-256 in reference.json (run kb-pack profile --inputs)"
            ));
        }
        out.push((n, input));
    }
    Ok(Some(out))
}

pub fn read_reference(dir: &Path) -> Result<Reference, String> {
    let path = reference_path(dir);
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// The cached input for each size (unverified; see [`cached_inputs`]).
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

/// Write the inputs into the cache folder (replacing it).
fn write_inputs(dir: &Path, sizes: &[u32], inputs: &[Map<String, Value>]) -> Result<(), String> {
    let folder = dir.join("profile").join("inputs");
    if folder.exists() {
        fs::remove_dir_all(&folder).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
    for (&n, input) in sizes.iter().zip(inputs) {
        fs::write(input_path(dir, n), input_text(input) + "\n").map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// `kb-pack profile --inputs` and `validate --run`: regenerate the inputs from gen.py,
/// check them (validate.py, growth, budget), verify each against its committed hash and
/// write the cache.
pub fn fill_cache(dir: &Path, env: &JudgeEnv, sizes: &[u32]) -> Result<String, String> {
    let reference = read_reference(dir)?;
    if reference.sizes != sizes || reference.sha256.len() != sizes.len() {
        return Err("profile/reference.json is for other sizes (run kb-pack profile)".into());
    }
    let fresh = generate(dir, env, sizes)?;
    for ((n, input), want) in sizes.iter().zip(&fresh).zip(&reference.sha256) {
        if sha256_hex(&input_text(input)) != *want {
            return Err(format!(
                "gen.py's input for n = {n} does not match its SHA-256 in reference.json"
            ));
        }
    }
    if let Some(e) = budget_error(&fresh) {
        return Err(e);
    }
    write_inputs(dir, sizes, &fresh)?;
    Ok(format!(
        "{} sizes regenerate from gen.py, match their hashes and pass validate.py",
        sizes.len()
    ))
}

/// The sizes of a profiled problem, from problem.toml.
fn profile_sizes(dir: &Path) -> Result<Vec<u32>, String> {
    let meta = crate::validate::read_problem_toml(dir)?;
    let Some(sizes) = meta.profile_sizes.filter(|_| meta.profile) else {
        return Err("problem.toml has no profile = true with profile_sizes".into());
    };
    if let Some(e) = sizes_error(&sizes) {
        return Err(e);
    }
    Ok(sizes)
}

/// `kb-pack profile --inputs DIR`: fill the input cache of the problem at `dir`, or of
/// every profiled problem under it. Stops at the first failure.
pub fn fill_caches(dir: &Path, env: &JudgeEnv) -> Result<Vec<String>, String> {
    let dirs = if dir.join("problem.toml").is_file() {
        vec![dir.to_path_buf()]
    } else {
        crate::validate::problem_dirs(dir).map_err(|e| format!("{}: {e}", dir.display()))?
    };
    let mut lines = Vec::new();
    for d in dirs {
        let Ok(meta) = crate::validate::read_problem_toml(&d) else {
            continue;
        };
        let Some(sizes) = meta.profile_sizes.filter(|_| meta.profile) else {
            continue;
        };
        let line = fill_cache(&d, env, &sizes).map_err(|e| format!("{}: {e}", d.display()))?;
        lines.push(format!("ok    {} {line}", meta.id));
    }
    Ok(lines)
}

/// `kb-pack profile DIR`: write the inputs and the reference times.
pub fn write_profile(dir: &Path, env: &JudgeEnv, sizes: &[u32]) -> Result<Vec<String>, String> {
    let problem = Problem::load(dir).map_err(|e| e.to_string())?;
    let inputs = generate(dir, env, sizes)?;
    if let Some(e) = budget_error(&inputs) {
        return Err(e);
    }
    write_inputs(dir, sizes, &inputs)?;
    let sha256 = inputs.iter().map(|m| sha256_hex(&input_text(m))).collect();
    let mut lines = Vec::new();
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
        seed: SEED,
        sha256,
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
    write_profile(dir, env, &profile_sizes(dir)?)
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

    /// two-sum's package without any cached inputs (CI has none either).
    fn two_sum_copy(name: &str) -> (PathBuf, Vec<u32>) {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/two-sum");
        let dir = std::env::temp_dir().join(format!("kbp-profile-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        crate::validate::tests::copy_dir(&src, &dir);
        let _ = fs::remove_dir_all(dir.join("profile").join("inputs"));
        let sizes = read_reference(&dir).unwrap().sizes;
        (dir, sizes)
    }

    #[test]
    fn the_reference_and_any_cached_inputs_are_checked() {
        let (dir, sizes) = two_sum_copy("files");
        // The committed reference alone is valid: no cache means no profile in the build.
        assert_eq!(files_error(&dir, &sizes), None);
        let reference = read_reference(&dir).unwrap();
        assert_eq!(reference.seed, SEED);
        assert_eq!(cached_inputs(&dir, &reference), Ok(None));

        // A cached input must match its hash; a matching cache is packed.
        let inputs: Vec<Map<String, Value>> = sizes
            .iter()
            .map(|&n| {
                serde_json::json!({ "nums": [n, 1], "target": 1 })
                    .as_object()
                    .unwrap()
                    .clone()
            })
            .collect();
        write_inputs(&dir, &sizes, &inputs).unwrap();
        assert!(
            files_error(&dir, &sizes)
                .unwrap()
                .contains("does not match its SHA-256")
        );
        let mut r = reference.clone();
        r.sha256 = inputs.iter().map(|m| sha256_hex(&input_text(m))).collect();
        fs::write(reference_path(&dir), serde_json::to_string(&r).unwrap()).unwrap();
        assert_eq!(files_error(&dir, &sizes), None);
        let cached = cached_inputs(&dir, &r).unwrap().unwrap();
        assert_eq!(cached.len(), sizes.len());
        assert_eq!(cached[0].1, inputs[0]);
        // A partial cache is no cache.
        fs::remove_file(input_path(&dir, sizes[2])).unwrap();
        assert_eq!(cached_inputs(&dir, &r), Ok(None));
        // An extra file is refused.
        fs::write(input_path(&dir, 7), "{}").unwrap();
        assert!(
            files_error(&dir, &sizes)
                .unwrap()
                .contains("not in profile_sizes")
        );

        // Sizes, hashes and times must cover every size and language.
        assert!(
            files_error(&dir, &sizes[1..])
                .unwrap()
                .contains("other sizes")
        );
        let mut bad = r.clone();
        bad.sha256.pop();
        fs::write(reference_path(&dir), serde_json::to_string(&bad).unwrap()).unwrap();
        assert!(files_error(&dir, &sizes).unwrap().contains("one SHA-256"));
        let mut bad = r.clone();
        bad.times_us.remove("php");
        fs::write(reference_path(&dir), serde_json::to_string(&bad).unwrap()).unwrap();
        assert!(
            files_error(&dir, &sizes)
                .unwrap()
                .contains("no times for php")
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_budget_counts_compressed_inputs() {
        let small = serde_json::json!({ "nums": [1, 2, 3] })
            .as_object()
            .unwrap()
            .clone();
        assert_eq!(budget_error(&[small]), None);
        // Incompressible numbers over 512 KB.
        let mut x: u64 = 1;
        let big: Vec<u64> = (0..250_000)
            .map(|_| {
                x = x
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                x >> 11
            })
            .collect();
        let big = serde_json::json!({ "nums": big })
            .as_object()
            .unwrap()
            .clone();
        assert!(budget_error(&[big]).unwrap().contains("budget"));
    }
}
