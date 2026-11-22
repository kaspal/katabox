//! Hostile packs (expansion M11, ADR 0038's threat model): every fixture is built here, in
//! the test, and must be refused for the right reason; a well-formed pack round-trips.
use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};

use kb_pack::kbpack::{self, archive};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("kb-pack-hostile-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

type Entry<'a> = (&'a str, Vec<u8>);

/// A zip of `entries` (deflate, 0o644), in order.
fn zip_of(entries: &[Entry]) -> Vec<u8> {
    let mut buf = std::io::Cursor::new(Vec::new());
    let mut z = zip::ZipWriter::new(&mut buf);
    let opts = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, data) in entries {
        z.start_file(*name, opts).unwrap();
        z.write_all(data).unwrap();
    }
    z.finish().unwrap();
    buf.into_inner()
}

fn extract_err(bytes: &[u8], name: &str) -> String {
    let dir = temp(name);
    archive::extract_bytes(bytes, &dir.join("src"))
        .expect_err(name)
        .to_string()
}

/// A copy of a real problem folder, to edit.
fn problem_copy(name: &str, id: &str) -> PathBuf {
    let root = temp(name);
    let dst = root.join(id);
    copy(&repo().join("content/problems").join(id), &dst);
    dst
}

fn copy(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).unwrap();
    for e in std::fs::read_dir(src).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            if p.file_name().unwrap() != "inputs" {
                copy(&p, &dst.join(p.file_name().unwrap()));
            }
        } else {
            std::fs::copy(&p, dst.join(p.file_name().unwrap())).unwrap();
        }
    }
}

fn manifest_file(dir: &Path, id: &str, min_app: &str) -> PathBuf {
    let m = dir.join("pack.toml");
    std::fs::write(
        &m,
        format!("id = \"{id}\"\nname = \"Graph drills\"\nauthor = \"Ana\"\nversion = \"1.0.0\"\nlicence = \"CC-BY-SA-4.0\"\nmin_app = \"{min_app}\"\n"),
    )
    .unwrap();
    m
}

/// Pack `problems`, then open the result like the app does.
fn pack_and_open(
    name: &str,
    problems: &[PathBuf],
    id: &str,
    builtin: &[&str],
) -> Result<kbpack::OpenedPack, Vec<String>> {
    let dir = temp(&format!("{name}-pack"));
    let out = dir.join("x.kbpack");
    kbpack::write_pack(problems, &manifest_file(&dir, id, "1.0.0-beta.1"), &out).unwrap();
    let src = dir.join("src");
    let files = archive::extract(&out, &src).map_err(|r| vec![r.to_string()])?;
    let builtin: BTreeSet<String> = builtin.iter().map(|s| s.to_string()).collect();
    kbpack::check_extracted(&src, &files, &builtin, "1.0.0-beta.1")
}

#[test]
fn a_well_formed_pack_round_trips_and_is_deterministic() {
    let p = problem_copy("good", "budget-pair");
    let pack = pack_and_open("good", std::slice::from_ref(&p), "drills", &["budget-pair"])
        .unwrap_or_else(|e| panic!("{e:#?}"));
    assert_eq!(pack.manifest.id, "drills");
    assert_eq!(pack.problems.len(), 1);
    let db = temp("good-db").join("content.db");
    assert_eq!(kbpack::build_pack_db(&pack, &db).unwrap(), 1);
    // The same input gives the same bytes.
    let d = temp("determinism");
    let m = manifest_file(&d, "drills", "1.0.0");
    let a = kbpack::write_pack(std::slice::from_ref(&p), &m, &d.join("a.kbpack")).unwrap();
    let b = kbpack::write_pack(std::slice::from_ref(&p), &m, &d.join("b.kbpack")).unwrap();
    assert_eq!(a, b);
}

#[test]
fn t1_names_that_escape_or_confuse_are_refused() {
    let ok = b"x".to_vec();
    for (name, why) in [
        ("../evil.md", "`..`"),
        ("/abs.md", "absolute"),
        ("C:/drive.md", "only letters"),
        ("a\\b.md", "backslash"),
        ("problems/CON.md", "device"),
        ("problems/nul.txt.md", "device"),
        ("problems/a.md:stream", "only letters"),
        ("problems/.hidden.md", "dot"),
        ("a/b/c/d/e/f/g/h/i/deep.md", "too deep"),
    ] {
        let err = extract_err(
            &zip_of(&[(name, ok.clone())]),
            &format!("t1-{}", name.len()),
        );
        assert!(err.contains(why), "{name}: {err}");
    }
    let long = format!("problems/{}.md", "a".repeat(190));
    assert!(extract_err(&zip_of(&[(long.as_str(), ok.clone())]), "t1-long").contains("too long"));
    let dup = zip_of(&[("problems/A.md", ok.clone()), ("problems/a.md", ok)]);
    assert!(extract_err(&dup, "t1-case").contains("case"));
}

#[test]
fn t2_links_and_special_files_are_refused() {
    let mut buf = std::io::Cursor::new(Vec::new());
    {
        let mut z = zip::ZipWriter::new(&mut buf);
        z.add_symlink(
            "problems/link.md",
            "../../outside",
            zip::write::SimpleFileOptions::default(),
        )
        .unwrap();
        z.finish().unwrap();
    }
    assert!(extract_err(&buf.into_inner(), "t2-link").contains("links"));
    // A character device: the zip writer normalises modes, so set the central directory's
    // "made by Unix" and external attributes (mode << 16) by hand.
    let mut bytes = zip_of(&[("problems/dev.md", b"x".to_vec())]);
    let cd = bytes
        .windows(4)
        .position(|w| w == [0x50, 0x4b, 0x01, 0x02])
        .unwrap();
    bytes[cd + 5] = 3;
    bytes[cd + 38..cd + 42].copy_from_slice(&(0o020644u32 << 16).to_le_bytes());
    let err = extract_err(&bytes, "t2-dev");
    assert!(err.contains("plain files"), "{err}");
}

#[test]
fn t3_bombs_and_oversized_files_are_refused() {
    // 5 MB of zeros in a test file: over the 4 MB cap (and a ratio far above 200:1).
    let bomb = zip_of(&[("problems/x/tests/hidden/1.json", vec![0u8; 5 * 1024 * 1024])]);
    assert!(bomb.len() < 64 * 1024, "the fixture really is compressed");
    let err = extract_err(&bomb, "t3-bomb");
    assert!(
        err.contains("larger than") || err.contains("200:1"),
        "{err}"
    );
    // A 300 KB statement: over the 256 KB text cap.
    let big = zip_of(&[("problems/x/statement.md", vec![b'a'; 300 * 1024])]);
    assert!(extract_err(&big, "t3-text").contains("larger than 256 KB"));
    // 3 MB of zeros as test JSON: under the size cap, but compressed far beyond 200:1.
    let ratio = zip_of(&[(
        "problems/x/tests/hidden/2.json",
        vec![b'0'; 3 * 1024 * 1024],
    )]);
    assert!(extract_err(&ratio, "t3-ratio").contains("200:1"));
    // Too many entries.
    let names: Vec<String> = (0..5001).map(|i| format!("problems/x/{i}.md")).collect();
    let many: Vec<Entry> = names.iter().map(|n| (n.as_str(), b"x".to_vec())).collect();
    assert!(extract_err(&zip_of(&many), "t3-many").contains("5000"));
}

#[test]
fn t4_only_text_file_types_are_accepted() {
    for name in [
        "problems/x/tool.exe",
        "problems/x/lib.dll",
        "problems/x/page.html",
        "problems/x/README",
        "problems/x/run.sh",
    ] {
        let err = extract_err(
            &zip_of(&[(name, b"x".to_vec())]),
            &format!("t4-{}", name.len()),
        );
        assert!(err.contains("file type not allowed"), "{name}: {err}");
    }
}

#[test]
fn t5_hashes_must_match_the_manifest_exactly() {
    let p = problem_copy("t5", "budget-pair");
    let dir = temp("t5-pack");
    let out = dir.join("x.kbpack");
    kbpack::write_pack(
        std::slice::from_ref(&p),
        &manifest_file(&dir, "drills", "1.0.0"),
        &out,
    )
    .unwrap();
    let open = |name: &str, change: &dyn Fn(&Path)| {
        let src = temp(name).join("src");
        let files = archive::extract(&out, &src).unwrap();
        let mut files = files;
        change(&src);
        // Re-hash what is on disk now, as if the zip had carried it.
        for (path, sha) in files.iter_mut() {
            if let Ok(bytes) = std::fs::read(src.join(path)) {
                *sha = archive::hex(&<sha2::Sha256 as sha2::Digest>::digest(&bytes));
            }
        }
        files.retain(|p, _| src.join(p).is_file());
        for e in walk(&src) {
            let rel = e
                .strip_prefix(&src)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            files.entry(rel).or_insert_with(|| {
                archive::hex(&<sha2::Sha256 as sha2::Digest>::digest(
                    std::fs::read(&e).unwrap(),
                ))
            });
        }
        kbpack::check_extracted(&src, &files, &BTreeSet::new(), "1.0.0")
            .unwrap_err()
            .join("\n")
    };
    let flipped = open("t5-flip", &|src| {
        let f = src.join("problems/budget-pair/statement.md");
        let mut b = std::fs::read(&f).unwrap();
        b[0] ^= 1;
        std::fs::write(f, b).unwrap();
    });
    assert!(flipped.contains("SHA-256 does not match"), "{flipped}");
    let extra = open("t5-extra", &|src| {
        std::fs::write(src.join("problems/budget-pair/extra.md"), "x").unwrap()
    });
    assert!(extra.contains("not listed in the manifest"), "{extra}");
    let missing = open("t5-missing", &|src| {
        std::fs::remove_file(src.join("problems/budget-pair/hints.md")).unwrap()
    });
    assert!(
        missing.contains("listed in the manifest but missing"),
        "{missing}"
    );
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(walk(&p))
        } else {
            out.push(p)
        }
    }
    out
}

#[test]
fn t6_t10_bad_metadata_and_spoofed_ids_are_refused() {
    let p = problem_copy("t6", "budget-pair");
    // T10: a built-in id, or the core pack's.
    let err = pack_and_open(
        "t10-builtin",
        std::slice::from_ref(&p),
        "anagrams",
        &["anagrams"],
    )
    .unwrap_err()
    .join("\n");
    assert!(err.contains("taken by built-in content"), "{err}");
    let err = pack_and_open("t10-core", std::slice::from_ref(&p), "core", &[])
        .unwrap_err()
        .join("\n");
    assert!(err.contains("taken by built-in content"), "{err}");
    // T6: an id that could look like a problem id, a future app version.
    let err = pack_and_open("t6-id", std::slice::from_ref(&p), "two-sum", &[])
        .unwrap_err()
        .join("\n");
    assert!(err.contains("3-20 lowercase"), "{err}");
    let dir = temp("t6-minapp");
    let out = dir.join("x.kbpack");
    kbpack::write_pack(
        std::slice::from_ref(&p),
        &manifest_file(&dir, "drills", "99.0.0"),
        &out,
    )
    .unwrap();
    let src = dir.join("src");
    let files = archive::extract(&out, &src).unwrap();
    let err = kbpack::check_extracted(&src, &files, &BTreeSet::new(), "1.0.0-beta.1")
        .unwrap_err()
        .join("\n");
    assert!(err.contains("needs KataBox 99.0.0"), "{err}");
    // T6: the same structural rules as built-in content (a test that does not parse).
    let broken = problem_copy("t6-struct", "budget-pair");
    std::fs::write(broken.join("tests/public/1.json"), "{ not json").unwrap();
    let err = pack_and_open("t6-struct", &[broken], "drills", &[])
        .unwrap_err()
        .join("\n");
    assert!(err.contains("problems/budget-pair"), "{err}");
}

#[test]
fn t7_markup_in_pack_text_is_refused() {
    for (i, evil) in [
        "<img src=x onerror=alert(1)>",
        "[click](javascript:alert(1))",
        "![x](data:image/png;base64,AAAA)",
        "<script>fetch('http://example.invalid')</script>",
        "<iframe src=\"http://example.invalid\"></iframe>",
    ]
    .iter()
    .enumerate()
    {
        let p = problem_copy(&format!("t7-{i}"), "budget-pair");
        let s = std::fs::read_to_string(p.join("statement.md")).unwrap();
        std::fs::write(p.join("statement.md"), format!("{s}\n{evil}\n")).unwrap();
        let err = pack_and_open(&format!("t7-{i}"), &[p], "drills", &[])
            .unwrap_err()
            .join("\n");
        assert!(err.contains("not allowed in pack text"), "{evil}: {err}");
    }
}

#[test]
fn t9_signature_names_must_be_identifiers() {
    let p = problem_copy("t9", "budget-pair");
    let toml = std::fs::read_to_string(p.join("problem.toml")).unwrap();
    let evil = toml.replacen(
        "name = \"budget\"",
        "name = \"budget\\\"); import os #\"",
        1,
    );
    assert_ne!(evil, toml, "the fixture really changes a parameter name");
    std::fs::write(p.join("problem.toml"), evil).unwrap();
    let err = pack_and_open("t9", &[p], "drills", &[])
        .unwrap_err()
        .join("\n");
    assert!(err.contains("ASCII identifier"), "{err}");
}

/// Pack `problems` with `assignment` (classroom mode, expansion M12), then open it.
fn pack_with_assignment(
    name: &str,
    problems: &[PathBuf],
    assignment: &str,
    builtin: &[&str],
) -> Result<kbpack::OpenedPack, Vec<String>> {
    let dir = temp(&format!("{name}-pack"));
    let out = dir.join("x.kbpack");
    let a = dir.join("assignment.toml");
    std::fs::write(&a, assignment).unwrap();
    kbpack::write_pack_with(
        problems,
        &manifest_file(&dir, "week", "1.0.0-beta.1"),
        Some(&a),
        &out,
    )
    .unwrap();
    let src = dir.join("src");
    let files = archive::extract(&out, &src).map_err(|r| vec![r.to_string()])?;
    let builtin: BTreeSet<String> = builtin.iter().map(|s| s.to_string()).collect();
    kbpack::check_extracted(&src, &files, &builtin, "1.0.0-beta.1")
}

#[test]
fn assignments_resolve_pack_and_built_in_problems() {
    let p = problem_copy("assign", "budget-pair");
    let pack = pack_with_assignment(
        "assign",
        std::slice::from_ref(&p),
        "title = \"Week 3\"\ndue = \"2026-12-01\"\nproblems = [\"budget-pair\", \"two-sum\"]\nlanguages = [\"python\", \"rust\", \"python\"]\ntime_limit_min = 90\nhints = false\n",
        &["two-sum"],
    )
    .unwrap_or_else(|e| panic!("{e:#?}"));
    let a = pack.assignment.unwrap();
    assert_eq!(a.problems, ["week--budget-pair", "two-sum"]);
    assert_eq!(
        a.languages,
        [kb_core::Language::Python, kb_core::Language::Rust]
    );
    assert_eq!((a.time_limit_min, a.hints), (Some(90), false));
    // An assignment of built-in problems only needs no problems/ folder.
    let pack = pack_with_assignment(
        "assign-only",
        &[],
        "title = \"Warm-up\"\ndue = \"2026-12-01\"\nproblems = [\"two-sum\"]\nlanguages = [\"typescript\"]\n",
        &["two-sum"],
    )
    .unwrap_or_else(|e| panic!("{e:#?}"));
    assert!(pack.problems.is_empty());
    assert!(
        pack.assignment.as_ref().unwrap().hints,
        "hints default to on"
    );
    let db = temp("assign-only-db").join("content.db");
    assert_eq!(kbpack::build_pack_db(&pack, &db).unwrap(), 0);
}

#[test]
fn bad_assignments_are_refused() {
    for (i, (text, want)) in [
        ("title = \"x\"\ndue = \"2026-13-01\"\nproblems = [\"two-sum\"]\nlanguages = [\"python\"]\n", "due"),
        ("title = \"x\"\ndue = \"2026-12-01\"\nproblems = [\"nope\"]\nlanguages = [\"python\"]\n", "neither a problem"),
        ("title = \"x\"\ndue = \"2026-12-01\"\nproblems = [\"two-sum\", \"two-sum\"]\nlanguages = [\"python\"]\n", "listed twice"),
        ("title = \"x\"\ndue = \"2026-12-01\"\nproblems = [\"two-sum\"]\nlanguages = [\"cobol\"]\n", "not a judge language"),
        ("title = \"x\"\ndue = \"2026-12-01\"\nproblems = [\"two-sum\"]\nlanguages = []\n", "at least one"),
        ("title = \"x\"\ndue = \"2026-12-01\"\nproblems = [\"two-sum\"]\nlanguages = [\"python\"]\ntime_limit_min = 1\n", "5-600"),
        ("title = \"x\\u0007\"\ndue = \"2026-12-01\"\nproblems = [\"two-sum\"]\nlanguages = [\"python\"]\n", "control"),
        ("title = \"x\"\ndue = \"2026-12-01\"\nproblems = [\"two-sum\"]\nlanguages = [\"python\"]\nrun = \"cmd\"\n", "unknown field"),
        ("title = \"x\"\ndue = \"2026-12-01\"\nproblems = []\nlanguages = [\"python\"]\n", "problems: 1-50"),
    ]
    .iter()
    .enumerate()
    {
        let err = pack_with_assignment(&format!("bad-assign-{i}"), &[], text, &["two-sum"])
            .unwrap_err()
            .join("\n");
        assert!(err.contains("assignment.toml") && err.contains(want), "{i}: {err}");
    }
}
