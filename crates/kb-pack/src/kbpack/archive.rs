//! The `.kbpack` reader (expansion M11, ADR 0038): a zip of plain text files, extracted
//! into a fresh staging folder under strict rules. This is new code beside kb-toolchain's
//! extractor (its rules are mirrored here, not shared), so toolchain verification is not
//! touched.
//!
//! - T1 names: UTF-8, `[A-Za-z0-9._-]` segments, no `..`, no absolute paths, drive letters,
//!   backslashes, streams or reserved device names, at most 8 levels and 180 characters,
//!   no two names equal ignoring case; files are created with `create_new` only.
//! - T2 types: regular files and folders only (no links, devices, or other mode bits).
//! - T3 sizes: the archive at most 64 MB and 5000 entries; each file counted on its
//!   actually decompressed bytes (a capped reader, never the header): 4 MB for test JSON,
//!   256 KB otherwise, 128 MB in all; stored or deflate only; at most 200:1 for files over
//!   64 KB.
//! - T4 types of content: an extension allow-list.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use sha2::{Digest, Sha256};

pub const MAX_ARCHIVE_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_ENTRIES: usize = 5000;
pub const MAX_TEST_BYTES: u64 = 4 * 1024 * 1024;
pub const MAX_TEXT_BYTES: u64 = 256 * 1024;
pub const MAX_TOTAL_BYTES: u64 = 128 * 1024 * 1024;
pub const MAX_RATIO: u64 = 200;
const RATIO_FROM_BYTES: u64 = 64 * 1024;
const MAX_DEPTH: usize = 8;
const MAX_NAME: usize = 180;
pub const ALLOWED_EXTENSIONS: [&str; 8] = ["toml", "md", "json", "py", "ts", "php", "rs", "sql"];

/// Why an archive was refused: the entry and the rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    pub entry: String,
    pub why: String,
}

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.entry.is_empty() {
            write!(f, "{}", self.why)
        } else {
            write!(f, "{}: {}", self.entry, self.why)
        }
    }
}

fn refuse(entry: &str, why: impl Into<String>) -> Refused {
    Refused {
        entry: entry.into(),
        why: why.into(),
    }
}

/// T1: the path segments of an entry name, or why it is refused.
pub fn entry_parts(name: &str) -> Result<Vec<String>, &'static str> {
    if name.is_empty() || name.len() > MAX_NAME {
        return Err("name empty or too long");
    }
    if name.contains('\\') {
        return Err("backslash in name");
    }
    if name.starts_with('/') {
        return Err("absolute path");
    }
    let mut out = Vec::new();
    for part in name.split('/') {
        match part {
            "" => continue,
            "." | ".." => return Err("`.` or `..` in path"),
            p => {
                if !p
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
                {
                    return Err("a name may hold only letters, digits, `.`, `_` and `-`");
                }
                if p.ends_with('.') || p.starts_with('.') {
                    return Err("a name may not start or end with a dot");
                }
                let stem = p.split('.').next().unwrap_or("").to_ascii_uppercase();
                let device = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
                    || ((stem.starts_with("COM") || stem.starts_with("LPT"))
                        && stem.len() == 4
                        && stem.as_bytes()[3].is_ascii_digit());
                if device {
                    return Err("reserved device name");
                }
                out.push(p.to_string());
            }
        }
    }
    if out.is_empty() {
        return Err("empty name");
    }
    if out.len() > MAX_DEPTH {
        return Err("too deep");
    }
    Ok(out)
}

/// The extracted files: path → SHA-256 (hex), in name order.
pub type Extracted = BTreeMap<String, String>;

/// Extract `archive` into `staging` (a new, empty folder), refusing anything outside the
/// rules. Returns every file with its hash.
pub fn extract(archive: &Path, staging: &Path) -> Result<Extracted, Refused> {
    let meta =
        fs::metadata(archive).map_err(|e| refuse("", format!("cannot read the pack: {e}")))?;
    if meta.len() > MAX_ARCHIVE_BYTES {
        return Err(refuse("", "the pack is larger than 64 MB"));
    }
    let bytes = fs::read(archive).map_err(|e| refuse("", format!("cannot read the pack: {e}")))?;
    extract_bytes(&bytes, staging)
}

pub fn extract_bytes(bytes: &[u8], staging: &Path) -> Result<Extracted, Refused> {
    if bytes.len() as u64 > MAX_ARCHIVE_BYTES {
        return Err(refuse("", "the pack is larger than 64 MB"));
    }
    let mut ar = zip::ZipArchive::new(std::io::Cursor::new(bytes))
        .map_err(|e| refuse("", format!("not a zip file: {e}")))?;
    if ar.len() > MAX_ENTRIES {
        return Err(refuse("", "more than 5000 entries"));
    }
    fs::create_dir_all(staging).map_err(|e| refuse("", format!("staging: {e}")))?;
    let mut seen_folded = BTreeSet::new();
    let mut out = Extracted::new();
    let mut total = 0u64;
    for i in 0..ar.len() {
        let mut entry = ar
            .by_index(i)
            .map_err(|e| refuse(&format!("#{i}"), format!("unreadable entry: {e}")))?;
        let name = String::from_utf8(entry.name_raw().to_vec())
            .map_err(|_| refuse(&format!("#{i}"), "name is not UTF-8"))?;
        if entry.encrypted() {
            return Err(refuse(&name, "encrypted entries are not allowed"));
        }
        let parts = entry_parts(&name).map_err(|why| refuse(&name, why))?;
        let path = parts.join("/");
        if !seen_folded.insert(path.to_ascii_lowercase()) {
            return Err(refuse(&name, "two entries differ only in case (or repeat)"));
        }
        // T2: plain files and folders only.
        if entry.is_symlink() {
            return Err(refuse(&name, "links are not allowed"));
        }
        if let Some(mode) = entry.unix_mode() {
            let kind = mode & 0o170000;
            if kind != 0 && kind != 0o100000 && kind != 0o040000 {
                return Err(refuse(&name, "only plain files and folders are allowed"));
            }
        }
        if entry.is_dir() {
            continue;
        }
        if !entry.is_file() {
            return Err(refuse(&name, "only plain files and folders are allowed"));
        }
        if !matches!(
            entry.compression(),
            zip::CompressionMethod::Stored | zip::CompressionMethod::Deflated
        ) {
            return Err(refuse(&name, "only stored or deflate entries are allowed"));
        }
        // T4: content types.
        let ext = parts
            .last()
            .and_then(|p| p.rsplit_once('.'))
            .map(|(_, e)| e.to_ascii_lowercase());
        if !ext
            .as_deref()
            .is_some_and(|e| ALLOWED_EXTENSIONS.contains(&e))
        {
            return Err(refuse(
                &name,
                format!(
                    "file type not allowed (only .{})",
                    ALLOWED_EXTENSIONS.join(", .")
                ),
            ));
        }
        // T3: the real size, read through a cap.
        let cap = if ext.as_deref() == Some("json") && parts.iter().any(|p| p == "tests") {
            MAX_TEST_BYTES
        } else {
            MAX_TEXT_BYTES
        };
        let compressed = entry.compressed_size();
        let mut data = Vec::new();
        (&mut entry)
            .take(cap + 1)
            .read_to_end(&mut data)
            .map_err(|e| refuse(&name, format!("cannot read: {e}")))?;
        let size = data.len() as u64;
        if size > cap {
            return Err(refuse(&name, format!("larger than {} KB", cap / 1024)));
        }
        if size > RATIO_FROM_BYTES && size > compressed.max(1).saturating_mul(MAX_RATIO) {
            return Err(refuse(&name, "compressed more than 200:1"));
        }
        total += size;
        if total > MAX_TOTAL_BYTES {
            return Err(refuse(&name, "the pack unpacks to more than 128 MB"));
        }
        // Write under the staging folder only, never over an existing file.
        let mut target = staging.to_path_buf();
        for p in &parts {
            target.push(p);
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| refuse(&name, format!("cannot create the folder: {e}")))?;
        }
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&target)
            .map_err(|e| refuse(&name, format!("cannot create: {e}")))?;
        f.write_all(&data)
            .map_err(|e| refuse(&name, format!("cannot write: {e}")))?;
        out.insert(path, hex(&Sha256::digest(&data)));
    }
    Ok(out)
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// SHA-256 of a whole file (the pack's own hash, shown before import).
pub fn sha256_file(path: &Path) -> std::io::Result<String> {
    Ok(hex(&Sha256::digest(fs::read(path)?)))
}
