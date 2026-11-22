//! Problem packs (expansion M11, ADR 0038): `.kbpack` files anyone can make and share.
//! A pack is a zip of `manifest.toml` plus `problems/<id>/` folders in the ordinary
//! package format. Opening one runs the threat model's checks in order:
//! T1-T4 in [`archive`] (names, types, sizes, extensions), then here T5 (every file
//! listed with its SHA-256, nothing more or less), T6 (the manifest's fields and the same
//! structural validation as built-in content, test budget included), T7 (no raw HTML or
//! script URLs in the Markdown, as defence in depth: the app renders pack text without
//! links or images anyway), T9 (signatures are ASCII identifiers: the parser refuses
//! anything else) and T10 (the pack id is namespaced and may not be a built-in id).
//! Pack problems are stored as `<pack>--<id>`.
//!
//! A pack may also hold an `assignment.toml` at its root (classroom mode, expansion M12,
//! ADR 0039): a title, a due date, the problems (the pack's own or built-in ones), the
//! allowed languages, an optional time limit and whether hints stay on. Such a pack may
//! have no problems of its own.

pub mod archive;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};

pub const FORMAT: &str = "katabox-pack/v1";
pub const MANIFEST: &str = "manifest.toml";
pub const ASSIGNMENT: &str = "assignment.toml";
/// At most this many problems in an assignment.
pub const MAX_ASSIGNMENT_PROBLEMS: usize = 50;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: String,
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    /// An SPDX licence expression, as stated by the author.
    pub licence: String,
    /// The oldest app version that can open the pack.
    pub min_app: String,
    /// Every other file in the pack, with its SHA-256 (hex).
    pub files: BTreeMap<String, String>,
}

/// `MAJOR.MINOR.PATCH` with an optional `-pre.release`: comparable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

impl Version {
    pub fn parse(s: &str) -> Option<Version> {
        let (core, pre) = match s.split_once('-') {
            Some((c, p))
                if !p.is_empty() && p.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'.') =>
            {
                (c, Some(p.to_string()))
            }
            Some(_) => return None,
            None => (s, None),
        };
        let mut it = core.split('.');
        let mut num = || {
            it.next()
                .filter(|n| !n.is_empty() && n.len() <= 9 && n.bytes().all(|b| b.is_ascii_digit()))?
                .parse()
                .ok()
        };
        let v = Version {
            major: num()?,
            minor: num()?,
            patch: num()?,
            pre,
        };
        it.next().is_none().then_some(v)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        (self.major, self.minor, self.patch)
            .cmp(&(o.major, o.minor, o.patch))
            // A pre-release comes before its release.
            .then_with(|| match (&self.pre, &o.pre) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(a), Some(b)) => a.cmp(b),
            })
    }
}

/// `[a-z][a-z0-9]{2,19}`: no `-`, so `<pack>--<id>` never looks like a built-in id.
pub fn valid_pack_id(id: &str) -> bool {
    (3..=20).contains(&id.len())
        && id.as_bytes()[0].is_ascii_lowercase()
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
}

fn plain_text(s: &str, max: usize) -> bool {
    !s.trim().is_empty() && s.chars().count() <= max && !s.chars().any(char::is_control)
}

/// T7: Markdown that could reach a browser engine as markup or a script URL.
pub fn markdown_problem(text: &str) -> Option<&'static str> {
    let lower = text.to_ascii_lowercase();
    for (needle, why) in [
        ("<script", "a <script> tag"),
        ("<iframe", "an <iframe> tag"),
        ("<object", "an <object> tag"),
        ("<embed", "an <embed> tag"),
        ("javascript:", "a javascript: URL"),
        ("vbscript:", "a vbscript: URL"),
        ("data:", "a data: URL"),
    ] {
        if lower.contains(needle) {
            return Some(why);
        }
    }
    // An event handler inside a tag: `<img onerror=…>`.
    let b = lower.as_bytes();
    let mut in_tag = false;
    for (i, &c) in b.iter().enumerate() {
        match c {
            b'<' => in_tag = true,
            b'>' => in_tag = false,
            b'o' if in_tag
                && b.get(i + 1) == Some(&b'n')
                && i > 0
                && b[i - 1].is_ascii_whitespace() =>
            {
                let rest = &lower[i + 2..];
                let name_len = rest.bytes().take_while(u8::is_ascii_lowercase).count();
                if name_len > 0 && rest[name_len..].trim_start().starts_with('=') {
                    return Some("an event handler attribute");
                }
            }
            _ => {}
        }
    }
    None
}

/// `assignment.toml` as written by a teacher.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct AssignmentToml {
    title: String,
    due: String,
    problems: Vec<String>,
    languages: Vec<String>,
    time_limit_min: Option<u32>,
    #[serde(default = "yes")]
    hints: bool,
}

fn yes() -> bool {
    true
}

/// Read and check `assignment.toml`. A problem names one of the pack's own problem folders
/// (stored as `<pack>--<id>`) or else a built-in problem.
pub fn parse_assignment(
    text: &str,
    pack_id: &str,
    own: &BTreeSet<String>,
    builtin: &BTreeSet<String>,
) -> Result<kb_store::classroom::AssignmentMeta, Vec<String>> {
    let a: AssignmentToml = toml::from_str(text).map_err(|e| vec![format!("{ASSIGNMENT}: {e}")])?;
    let mut errs: Vec<String> = Vec::new();
    if !plain_text(&a.title, 80) {
        errs.push("title: 1-80 characters, no control characters".into());
    }
    if a.due.len() != 10 || a.due.parse::<kb_core::Day>().is_err() {
        errs.push("due: a date YYYY-MM-DD".into());
    }
    if a.problems.is_empty() || a.problems.len() > MAX_ASSIGNMENT_PROBLEMS {
        errs.push(format!("problems: 1-{MAX_ASSIGNMENT_PROBLEMS} problem ids"));
    }
    let mut problems: Vec<String> = Vec::new();
    for p in &a.problems {
        let full = if own.contains(p) {
            format!("{pack_id}--{p}")
        } else if builtin.contains(p) {
            p.clone()
        } else {
            errs.push(format!(
                "problems: {p:?} is neither a problem of this pack nor a built-in problem"
            ));
            continue;
        };
        if problems.contains(&full) {
            errs.push(format!("problems: {p:?} is listed twice"));
        } else {
            problems.push(full);
        }
    }
    let mut languages = Vec::new();
    for l in &a.languages {
        match l.parse::<kb_core::Language>() {
            Ok(lang) if kb_judge::languages::SUPPORTED.contains(&lang) => {
                if !languages.contains(&lang) {
                    languages.push(lang);
                }
            }
            _ => errs.push(format!("languages: {l:?} is not a judge language")),
        }
    }
    if a.languages.is_empty() {
        errs.push("languages: at least one".into());
    }
    if let Some(m) = a.time_limit_min
        && !(5..=600).contains(&m)
    {
        errs.push("time_limit_min: 5-600 minutes".into());
    }
    if errs.is_empty() {
        Ok(kb_store::classroom::AssignmentMeta {
            title: a.title,
            due: a.due,
            problems,
            languages,
            time_limit_min: a.time_limit_min,
            hints: a.hints,
        })
    } else {
        Err(errs
            .into_iter()
            .map(|e| format!("{ASSIGNMENT}: {e}"))
            .collect())
    }
}

/// What a pack holds, read and checked: ready to show before importing.
#[derive(Debug, Clone)]
pub struct OpenedPack {
    pub manifest: Manifest,
    /// The staging folder it was extracted to.
    pub dir: PathBuf,
    /// Problem folders (`problems/<id>`), follow-ups included, in order.
    pub problems: Vec<PathBuf>,
    /// The assignment it carries (classroom mode), if any.
    pub assignment: Option<kb_store::classroom::AssignmentMeta>,
}

/// Every check after extraction (T5, T6, T7, T9, T10). `builtin` are the built-in problem
/// ids (a pack id may not equal one); `app_version` is this app's version.
pub fn check_extracted(
    dir: &Path,
    files: &archive::Extracted,
    builtin: &BTreeSet<String>,
    app_version: &str,
) -> Result<OpenedPack, Vec<String>> {
    let mut errs = Vec::new();
    let manifest: Manifest = match fs::read_to_string(dir.join(MANIFEST))
        .map_err(|e| e.to_string())
        .and_then(|s| toml::from_str(&s).map_err(|e| e.to_string()))
    {
        Ok(m) => m,
        Err(e) => return Err(vec![format!("{MANIFEST}: {e}")]),
    };
    // T6: the manifest itself.
    if manifest.format != FORMAT {
        errs.push(format!("format must be {FORMAT:?}"));
    }
    if !valid_pack_id(&manifest.id) {
        errs.push("id: 3-20 lowercase letters and digits, starting with a letter".into());
    }
    // T10: no spoofing of built-in problems or the core pack.
    if manifest.id == "core" || builtin.contains(&manifest.id) {
        errs.push(format!("id {:?} is taken by built-in content", manifest.id));
    }
    if !plain_text(&manifest.name, 80) || !plain_text(&manifest.author, 80) {
        errs.push("name and author: 1-80 characters, no control characters".into());
    }
    if !plain_text(&manifest.licence, 64)
        || !manifest
            .licence
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b" .+-()".contains(&b))
    {
        errs.push("licence: an SPDX expression such as CC-BY-SA-4.0".into());
    }
    if Version::parse(&manifest.version).is_none() {
        errs.push("version: MAJOR.MINOR.PATCH".into());
    }
    match (
        Version::parse(&manifest.min_app),
        Version::parse(app_version),
    ) {
        (Some(min), Some(app)) if min > app => {
            errs.push(format!(
                "needs KataBox {} or newer (this is {app_version})",
                manifest.min_app
            ));
        }
        (None, _) => errs.push("min_app: MAJOR.MINOR.PATCH".into()),
        _ => {}
    }
    // T5: exactly the listed files, each with its hash.
    for (path, sha) in files {
        if path == MANIFEST {
            continue;
        }
        match manifest.files.get(path) {
            None => errs.push(format!("{path}: not listed in the manifest")),
            Some(want) if !want.eq_ignore_ascii_case(sha) => {
                errs.push(format!("{path}: SHA-256 does not match the manifest"))
            }
            Some(_) => {}
        }
    }
    for path in manifest.files.keys() {
        if !files.contains_key(path) {
            errs.push(format!("{path}: listed in the manifest but missing"));
        }
    }
    // Layout: problems/ and an optional assignment.toml (Learn topics in packs are not
    // supported yet).
    for path in files.keys() {
        if path != MANIFEST && path != ASSIGNMENT && !path.starts_with("problems/") {
            errs.push(format!(
                "{path}: packs hold problems/ and assignment.toml only"
            ));
        }
        // T7.
        if path.ends_with(".md")
            && let Some(why) = fs::read_to_string(dir.join(path))
                .ok()
                .as_deref()
                .and_then(markdown_problem)
        {
            errs.push(format!("{path}: {why} is not allowed in pack text"));
        }
    }
    // T6 + T9: the same structural validation as built-in content.
    let root = dir.join("problems");
    let has_assignment = files.contains_key(ASSIGNMENT);
    let problems = if root.is_dir() {
        match crate::validate::problem_dirs(&root) {
            Ok(d) => d,
            Err(e) => {
                errs.push(format!("problems/: {e}"));
                Vec::new()
            }
        }
    } else {
        if !has_assignment {
            errs.push("the pack has no problems/ folder".into());
        }
        Vec::new()
    };
    if problems.is_empty() && root.is_dir() {
        errs.push("the pack has no problems".into());
    }
    for p in &problems {
        let id = crate::validate::problem_id_of(p);
        let (_, e) = crate::validate::validate_problem_meta(p);
        errs.extend(e.into_iter().map(|e| format!("problems/{id}: {e}")));
        match crate::validate::test_size(p) {
            Ok(size) => {
                if let Some(e) = crate::validate::budget_error(size) {
                    errs.push(format!("problems/{id}: {e}"));
                }
            }
            Err(e) => errs.push(format!("problems/{id}: {e}")),
        }
        let full = format!("{}--{id}", manifest.id);
        if full.len() > 64 {
            errs.push(format!(
                "problems/{id}: {full} is longer than 64 characters"
            ));
        }
    }
    let assignment = if has_assignment {
        let own: BTreeSet<String> = problems
            .iter()
            .map(|p| crate::validate::problem_id_of(p).to_string())
            .collect();
        match fs::read_to_string(dir.join(ASSIGNMENT))
            .map_err(|e| vec![format!("{ASSIGNMENT}: {e}")])
            .and_then(|t| parse_assignment(&t, &manifest.id, &own, builtin))
        {
            Ok(a) => Some(a),
            Err(e) => {
                errs.extend(e);
                None
            }
        }
    } else {
        None
    };
    if errs.is_empty() {
        Ok(OpenedPack {
            manifest,
            dir: dir.to_path_buf(),
            problems,
            assignment,
        })
    } else {
        Err(errs)
    }
}

/// A pack kept on disk after import (`src/`, already checked then): its manifest and
/// problem folders, to rebuild its content.db after an app update changed the schema.
pub fn reopen_kept(src: &Path) -> Result<OpenedPack, String> {
    let manifest: Manifest =
        toml::from_str(&fs::read_to_string(src.join(MANIFEST)).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let root = src.join("problems");
    let problems = if root.is_dir() {
        crate::validate::problem_dirs(&root).map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    Ok(OpenedPack {
        manifest,
        dir: src.to_path_buf(),
        problems,
        // Kept copies only rebuild problems; the assignment was recorded at import.
        assignment: None,
    })
}

/// Build the pack's own content.db at `out`: every problem as `<pack>--<id>`.
pub fn build_pack_db(pack: &OpenedPack, out: &Path) -> Result<usize, String> {
    let mut problems = Vec::new();
    for dir in &pack.problems {
        let mut p = crate::build::read_package_for_pack(dir)?;
        p.id = format!("{}--{}", pack.manifest.id, p.id);
        p.parent = p.parent.map(|base| format!("{}--{base}", pack.manifest.id));
        p.pack = Some(pack.manifest.id.clone());
        problems.push(p);
    }
    let meta = kb_store::PackMeta {
        pack_id: pack.manifest.id.clone(),
        pack_version: pack.manifest.version.clone(),
        pack_date: None,
    };
    crate::build::block_on(kb_store::build(out, &meta, &problems)).map_err(|e| e.to_string())?;
    Ok(problems.len())
}

/// Write a pack from problem folders: the manifest with every hash, then a zip with
/// sorted entries and a fixed timestamp, so the same input gives the same bytes.
pub fn write_pack(
    problem_dirs: &[PathBuf],
    manifest_toml: &Path,
    out: &Path,
) -> Result<String, String> {
    write_pack_with(problem_dirs, manifest_toml, None, out)
}

/// [`write_pack`] with an optional `assignment.toml` (classroom mode) at the pack's root.
pub fn write_pack_with(
    problem_dirs: &[PathBuf],
    manifest_toml: &Path,
    assignment: Option<&Path>,
    out: &Path,
) -> Result<String, String> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Head {
        id: String,
        name: String,
        author: String,
        version: String,
        licence: String,
        min_app: String,
    }
    let head: Head = toml::from_str(&fs::read_to_string(manifest_toml).map_err(|e| e.to_string())?)
        .map_err(|e| format!("{}: {e}", manifest_toml.display()))?;
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for dir in problem_dirs {
        let id = dir
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("bad folder name")?
            .to_string();
        let mut stack = vec![dir.clone()];
        while let Some(d) = stack.pop() {
            for e in fs::read_dir(&d).map_err(|e| e.to_string())? {
                let p = e.map_err(|e| e.to_string())?.path();
                let rel = p.strip_prefix(dir).map_err(|e| e.to_string())?;
                let parts: Vec<String> = rel
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect();
                if p.is_dir() {
                    // Generated caches stay out (ADR 0024).
                    if parts.last().map(String::as_str) != Some("inputs") {
                        stack.push(p);
                    }
                    continue;
                }
                let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                if !archive::ALLOWED_EXTENSIONS.contains(&ext) {
                    continue;
                }
                files.insert(
                    format!("problems/{id}/{}", parts.join("/")),
                    fs::read(&p).map_err(|e| e.to_string())?,
                );
            }
        }
    }
    if let Some(a) = assignment {
        files.insert(
            ASSIGNMENT.to_string(),
            fs::read(a).map_err(|e| format!("{}: {e}", a.display()))?,
        );
    }
    let mut manifest = format!(
        "format = {FORMAT:?}\nid = {:?}\nname = {:?}\nauthor = {:?}\nversion = {:?}\nlicence = {:?}\nmin_app = {:?}\n\n[files]\n",
        head.id, head.name, head.author, head.version, head.licence, head.min_app
    );
    for (path, data) in &files {
        manifest.push_str(&format!(
            "{path:?} = {:?}\n",
            archive::hex(&Sha256::digest(data))
        ));
    }
    let mut buf = std::io::Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut buf);
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .last_modified_time(zip::DateTime::default())
            .unix_permissions(0o644);
        zip.start_file(MANIFEST, opts).map_err(|e| e.to_string())?;
        zip.write_all(manifest.as_bytes())
            .map_err(|e| e.to_string())?;
        for (path, data) in &files {
            zip.start_file(path.as_str(), opts)
                .map_err(|e| e.to_string())?;
            zip.write_all(data).map_err(|e| e.to_string())?;
        }
        zip.finish().map_err(|e| e.to_string())?;
    }
    let bytes = buf.into_inner();
    fs::write(out, &bytes).map_err(|e| e.to_string())?;
    Ok(archive::hex(&Sha256::digest(&bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_with_pre_releases_first() {
        let v = |s| Version::parse(s).unwrap();
        assert!(v("1.0.0") > v("1.0.0-beta.1"));
        assert!(v("1.2.0") < v("1.10.0-rc.1"));
        assert!(v("2.0.0") > v("1.99.99"));
        for bad in ["1.0", "1.0.0.0", "a.b.c", "1.0.0-", "01x.0.0", ""] {
            assert!(Version::parse(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn pack_ids_and_markdown() {
        assert!(valid_pack_id("graphs101"));
        for bad in [
            "ab",
            "two-sum",
            "Graphs",
            "1abc",
            "a_b_c",
            "abcdefghijklmnopqrstu",
        ] {
            assert!(!valid_pack_id(bad), "{bad}");
        }
        assert_eq!(markdown_problem("A loop: `for x in xs`. One = 1."), None);
        assert!(markdown_problem("<img src=x onerror=alert(1)>").is_some());
        assert!(markdown_problem("[x](javascript:alert(1))").is_some());
        assert!(markdown_problem("![x](data:image/png;base64,AAAA)").is_some());
        assert!(markdown_problem("<SCRIPT>bad()</SCRIPT>").is_some());
    }
}
