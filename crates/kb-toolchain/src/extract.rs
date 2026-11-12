//! Safe extraction (ADR 0015). Runs only on a verified download, re-verified from the same
//! open handle. Writes only regular files and folders into a fresh staging folder; every
//! entry name is checked, links are never created (listed in-archive links are copied as
//! files), and the bytes written are capped.

use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use tokio_util::sync::CancellationToken;

use crate::download::verify_reader;
use crate::layout::is_link;
use crate::manifest::{Artifact, Format, Source, hex};
use crate::{Error, Result};

/// The most entries a component may have (rust-src and basedpyright have ~5,000).
const MAX_ENTRIES: u64 = 200_000;

/// What extraction may still write for one component.
#[derive(Debug)]
pub(crate) struct Budget {
    pub bytes: u64,
    pub entries: u64,
}

impl Budget {
    /// 110% of the expected installed size, plus a little slack.
    pub fn for_size(installed: u64) -> Budget {
        Budget {
            bytes: installed + installed / 10 + (1 << 20),
            entries: MAX_ENTRIES,
        }
    }
}

enum Kind {
    Dir,
    File,
    Symlink(Vec<u8>),
    Hardlink(Vec<u8>),
}

/// A link to copy once every file is extracted.
struct PendingLink {
    rel: Vec<String>,
    target: Vec<String>,
}

/// Open a verified download without allowing writers (Windows), re-verify it from that
/// handle and rewind, so what is extracted is what was checked.
fn open_verified(path: &Path, artifact: &Artifact) -> Result<File> {
    let mut opts = OpenOptions::new();
    opts.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_SHARE_READ: u32 = 0x1;
        opts.share_mode(FILE_SHARE_READ);
    }
    let mut f = opts
        .open(path)
        .map_err(|e| Error::io(format!("open {}", path.display()), e))?;
    verify_reader(&mut BufReader::new(&mut f), artifact)?;
    f.seek(SeekFrom::Start(0))
        .map_err(|e| Error::io("rewind", e))?;
    Ok(f)
}

/// Extract one source of a component into `dest` (the component's staging folder).
pub(crate) fn extract_source(
    file: &Path,
    artifact: &Artifact,
    source: &Source,
    dest: &Path,
    budget: &mut Budget,
    cancel: &CancellationToken,
) -> Result<()> {
    let f = open_verified(file, artifact)?;
    let mut x = Extractor {
        artifact,
        source,
        dest,
        budget,
        cancel,
        links: Vec::new(),
        member_found: false,
        written: HashSet::new(),
    };
    match artifact.format {
        Format::TarGz => x.tar(f)?,
        Format::Zip => x.zip(f)?,
    }
    if let Some(member) = &source.member
        && !x.member_found
    {
        return Err(x.unsafe_entry(member, "member not found in the archive"));
    }
    x.copy_links()
}

struct Extractor<'a> {
    artifact: &'a Artifact,
    source: &'a Source,
    dest: &'a Path,
    budget: &'a mut Budget,
    cancel: &'a CancellationToken,
    links: Vec<PendingLink>,
    member_found: bool,
    /// Regular files written (relative, `/`-joined), for link targets.
    written: HashSet<String>,
}

impl Extractor<'_> {
    fn unsafe_entry(&self, entry: &str, why: &str) -> Error {
        Error::UnsafeArchive {
            artifact: self.artifact.id.clone(),
            entry: entry.to_string(),
            why: why.to_string(),
        }
    }

    fn tar(&mut self, f: File) -> Result<()> {
        // Hash again while streaming: what is unpacked must be what was verified, even if
        // the file changed on disk after the check (Linux has no deny-write sharing).
        let mut hashing = HashingReader::new(BufReader::new(f));
        {
            let gz = flate2::read::GzDecoder::new(&mut hashing);
            let mut ar = tar::Archive::new(gz);
            let entries = ar
                .entries()
                .map_err(|e| self.unsafe_entry("", &format!("not a tar.gz: {e}")))?;
            for entry in entries {
                if self.cancel.is_cancelled() {
                    return Err(Error::Cancelled);
                }
                let mut entry =
                    entry.map_err(|e| self.unsafe_entry("", &format!("corrupt archive: {e}")))?;
                let raw = entry.path_bytes().into_owned();
                let ty = entry.header().entry_type();
                let kind = if ty.is_dir() {
                    Kind::Dir
                } else if ty.is_file() || ty == tar::EntryType::Continuous {
                    Kind::File
                } else if ty.is_symlink() {
                    Kind::Symlink(
                        entry
                            .link_name_bytes()
                            .map(|c| c.into_owned())
                            .unwrap_or_default(),
                    )
                } else if ty.is_hard_link() {
                    Kind::Hardlink(
                        entry
                            .link_name_bytes()
                            .map(|c| c.into_owned())
                            .unwrap_or_default(),
                    )
                } else if ty.is_pax_global_extensions()
                    || ty.is_pax_local_extensions()
                    || ty.is_gnu_longname()
                    || ty.is_gnu_longlink()
                {
                    continue;
                } else {
                    let name = String::from_utf8_lossy(&raw).into_owned();
                    return Err(
                        self.unsafe_entry(&name, &format!("entry type {ty:?} is not allowed"))
                    );
                };
                let mode = entry.header().mode().unwrap_or(0o644);
                self.entry(&raw, kind, mode, &mut entry)?;
            }
        }
        // Drain the rest of the stream so the hash covers the whole file.
        io::copy(&mut hashing, &mut io::sink()).map_err(|e| Error::io("read archive", e))?;
        let (n, sha) = hashing.finish();
        if n != self.artifact.size || sha != self.artifact.sha256 {
            return Err(Error::Checksum {
                artifact: self.artifact.id.clone(),
                expected: self.artifact.sha256.clone(),
                actual: format!("{sha} ({n} bytes) while extracting"),
            });
        }
        Ok(())
    }

    fn zip(&mut self, f: File) -> Result<()> {
        let mut ar = zip::ZipArchive::new(BufReader::new(f))
            .map_err(|e| self.unsafe_entry("", &format!("not a zip: {e}")))?;
        for i in 0..ar.len() {
            if self.cancel.is_cancelled() {
                return Err(Error::Cancelled);
            }
            let mut entry = match ar.by_index(i) {
                Ok(e) => e,
                Err(e) => {
                    return Err(self.unsafe_entry(
                        &format!("#{i}"),
                        &format!("unreadable entry (encrypted or unsupported): {e}"),
                    ));
                }
            };
            if entry.encrypted() {
                let n = String::from_utf8_lossy(entry.name_raw()).into_owned();
                return Err(self.unsafe_entry(&n, "encrypted entries are not allowed"));
            }
            let raw = entry.name_raw().to_vec();
            let mode = entry.unix_mode().unwrap_or(0o644);
            let kind = if entry.is_symlink() {
                let mut target = Vec::new();
                (&mut entry)
                    .take(4096)
                    .read_to_end(&mut target)
                    .map_err(|e| Error::io("read link", e))?;
                Kind::Symlink(target)
            } else if entry.is_dir() {
                Kind::Dir
            } else if entry.is_file() {
                Kind::File
            } else {
                let n = String::from_utf8_lossy(&raw).into_owned();
                return Err(self.unsafe_entry(&n, "entry type is not allowed"));
            };
            self.entry(&raw, kind, mode, &mut entry)?;
        }
        Ok(())
    }

    fn entry(&mut self, raw: &[u8], kind: Kind, mode: u32, data: &mut dyn Read) -> Result<()> {
        let display = String::from_utf8_lossy(raw).into_owned();
        // Every entry is checked, even those outside the subtree we keep.
        let parts = entry_parts(raw).map_err(|why| self.unsafe_entry(&display, why))?;
        if self.budget.entries == 0 {
            return Err(self.unsafe_entry(&display, "too many entries"));
        }
        self.budget.entries -= 1;

        // A single member: written at the component root under its file name.
        if let Some(member) = &self.source.member {
            if parts.join("/") != *member {
                return Ok(());
            }
            if !matches!(kind, Kind::File) {
                return Err(self.unsafe_entry(&display, "member is not a regular file"));
            }
            let name = parts.last().cloned().unwrap_or_default();
            let sha = self.write_file(&[name], mode, data, &display)?;
            let want = self.source.member_sha256.clone().unwrap_or_default();
            if sha != want {
                return Err(Error::Checksum {
                    artifact: format!("{} ({member})", self.artifact.id),
                    expected: want,
                    actual: sha,
                });
            }
            self.member_found = true;
            return Ok(());
        }

        let subtree: Vec<&str> = self
            .source
            .subtree
            .as_deref()
            .unwrap_or("")
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        if parts.len() < subtree.len() || parts[..subtree.len()] != subtree[..] {
            return Ok(());
        }
        let rel: Vec<String> = parts[subtree.len()..].to_vec();
        if rel.is_empty() {
            return Ok(());
        }
        let rel_str = rel.join("/");
        if self.source.exclude.contains(&rel_str) {
            return Ok(());
        }
        match kind {
            Kind::Dir => {
                let p = self.path(&rel);
                fs::create_dir_all(&p)
                    .map_err(|e| Error::io(format!("create {}", p.display()), e))?;
                Ok(())
            }
            Kind::File => {
                self.write_file(&rel, mode, data, &display)?;
                self.written.insert(rel_str);
                Ok(())
            }
            Kind::Symlink(target) if self.source.links.contains(&rel_str) => {
                let target = self.symlink_target(&rel, &target, &display)?;
                self.links.push(PendingLink { rel, target });
                Ok(())
            }
            Kind::Hardlink(target) if self.source.links.contains(&rel_str) => {
                // A tar hard link names another archive path; it must be in our subtree.
                let t = entry_parts(&target).map_err(|why| self.unsafe_entry(&display, why))?;
                if t.len() <= subtree.len() || t[..subtree.len()] != subtree[..] {
                    return Err(
                        self.unsafe_entry(&display, "link target leaves the extracted folder")
                    );
                }
                self.links.push(PendingLink {
                    rel,
                    target: t[subtree.len()..].to_vec(),
                });
                Ok(())
            }
            Kind::Symlink(_) | Kind::Hardlink(_) if self.skipped_link(&rel_str) => Ok(()),
            Kind::Symlink(_) => Err(self.unsafe_entry(&display, "symbolic links are not allowed")),
            Kind::Hardlink(_) => Err(self.unsafe_entry(&display, "hard links are not allowed")),
        }
    }

    fn skipped_link(&self, rel: &str) -> bool {
        self.source.skip_links.iter().any(|s| {
            if s.ends_with('/') {
                rel.starts_with(s.as_str())
            } else {
                s == rel
            }
        })
    }

    /// Resolve a listed symlink's target (relative to the link's folder) inside the
    /// extracted tree; it must not leave it.
    fn symlink_target(&self, rel: &[String], target: &[u8], display: &str) -> Result<Vec<String>> {
        let t = std::str::from_utf8(target)
            .map_err(|_| self.unsafe_entry(display, "link target is not UTF-8"))?;
        if t.is_empty()
            || t.starts_with('/')
            || t.contains('\\')
            || t.contains('\0')
            || t.contains(':')
        {
            return Err(self.unsafe_entry(
                display,
                "link target must be relative and inside the archive",
            ));
        }
        let mut out: Vec<String> = rel[..rel.len() - 1].to_vec();
        for part in t.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    if out.pop().is_none() {
                        return Err(
                            self.unsafe_entry(display, "link target leaves the extracted folder")
                        );
                    }
                }
                p => out.push(p.to_string()),
            }
        }
        Ok(out)
    }

    /// Copy listed links' targets (regular files extracted from this archive).
    fn copy_links(&mut self) -> Result<()> {
        let links = std::mem::take(&mut self.links);
        for l in links {
            let display = l.rel.join("/");
            let target = l.target.join("/");
            if !self.written.contains(&target) {
                return Err(self.unsafe_entry(
                    &display,
                    "link target is not a regular file of this archive",
                ));
            }
            let src = self.path(&l.target);
            let meta =
                fs::symlink_metadata(&src).map_err(|e| Error::io("inspect link target", e))?;
            if !meta.is_file() || is_link(&meta) {
                return Err(self.unsafe_entry(&display, "link target is not a regular file"));
            }
            let mut f = File::open(&src).map_err(|e| Error::io("read link target", e))?;
            let mode = executable_mode(&meta);
            self.write_file(&l.rel, mode, &mut f, &display)?;
            self.written.insert(display);
        }
        Ok(())
    }

    fn path(&self, rel: &[String]) -> PathBuf {
        let mut p = self.dest.to_path_buf();
        for part in rel {
            p.push(part);
        }
        p
    }

    /// Create a new file (never overwrite), copy at most the remaining budget, set a safe
    /// mode. Returns the SHA-256 of what was written.
    fn write_file(
        &mut self,
        rel: &[String],
        mode: u32,
        data: &mut dyn Read,
        display: &str,
    ) -> Result<String> {
        let p = self.path(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| Error::io(format!("create {}", parent.display()), e))?;
        }
        let mut out = match OpenOptions::new().write(true).create_new(true).open(&p) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                return Err(self.unsafe_entry(
                    display,
                    "duplicate entry (or a name differing only in case)",
                ));
            }
            Err(e) => return Err(Error::io(format!("create {}", p.display()), e)),
        };
        let mut sha = Sha256::new();
        let mut buf = vec![0u8; 1 << 16];
        loop {
            let n = data
                .read(&mut buf)
                .map_err(|e| Error::io(format!("extract {display}"), e))?;
            if n == 0 {
                break;
            }
            if n as u64 > self.budget.bytes {
                return Err(self.unsafe_entry(display, "archive expands beyond its expected size"));
            }
            self.budget.bytes -= n as u64;
            out.write_all(&buf[..n])
                .map_err(|e| Error::io(format!("write {}", p.display()), e))?;
            sha.update(&buf[..n]);
        }
        out.flush()
            .map_err(|e| Error::io(format!("write {}", p.display()), e))?;
        drop(out);
        set_mode(&p, mode)?;
        Ok(hex(&sha.finalize()))
    }
}

#[cfg(unix)]
fn executable_mode(m: &fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    m.permissions().mode()
}
#[cfg(not(unix))]
fn executable_mode(_: &fs::Metadata) -> u32 {
    0o644
}

/// Unix: 0o755 if any execute bit was set, else 0o644 (never setuid/setgid). Windows: the
/// file keeps its defaults (never the readonly attribute, which would block deletes).
fn set_mode(p: &Path, mode: u32) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let m = if mode & 0o111 != 0 { 0o755 } else { 0o644 };
        fs::set_permissions(p, fs::Permissions::from_mode(m))
            .map_err(|e| Error::io(format!("chmod {}", p.display()), e))?;
    }
    #[cfg(not(unix))]
    let _ = (p, mode);
    Ok(())
}

/// Split an archive entry name into plain parts, or say why it is unsafe.
pub(crate) fn entry_parts(raw: &[u8]) -> std::result::Result<Vec<String>, &'static str> {
    let s = std::str::from_utf8(raw).map_err(|_| "name is not UTF-8")?;
    if s.is_empty() {
        return Err("empty name");
    }
    if s.bytes().any(|b| b < 0x20 || b == 0x7f) {
        return Err("control character in name");
    }
    if s.contains('\\') {
        return Err("backslash in name");
    }
    if s.starts_with('/') {
        return Err("absolute path");
    }
    let mut out = Vec::new();
    for part in s.split('/') {
        match part {
            "" | "." => continue,
            ".." => return Err("`..` in path"),
            p => {
                windows_name_ok(p)?;
                out.push(p.to_string());
            }
        }
    }
    Ok(out)
}

/// Names Windows would misread: drive letters and streams (`:`), wildcards, device names,
/// trailing dots or spaces. Checked on every platform so archives behave the same.
fn windows_name_ok(p: &str) -> std::result::Result<(), &'static str> {
    if p.chars()
        .any(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*'))
    {
        return Err("character not allowed in a file name");
    }
    if p.ends_with('.') || p.ends_with(' ') {
        return Err("name ends with a dot or space");
    }
    let stem = p
        .split('.')
        .next()
        .unwrap_or("")
        .trim_end()
        .to_ascii_uppercase();
    let reserved = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || ((stem.starts_with("COM") || stem.starts_with("LPT"))
            && stem.len() == 4
            && stem.as_bytes()[3].is_ascii_digit()
            && stem.as_bytes()[3] != b'0');
    if reserved {
        return Err("reserved device name");
    }
    Ok(())
}

/// After extraction: only plain files and folders, no links or reparse points.
pub(crate) fn assert_no_links(dir: &Path) -> Result<()> {
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in fs::read_dir(&d).map_err(|e| Error::io(format!("list {}", d.display()), e))? {
            let e = e.map_err(|e| Error::io(format!("list {}", d.display()), e))?;
            let m = fs::symlink_metadata(e.path()).map_err(|e| Error::io("inspect", e))?;
            if is_link(&m) {
                return Err(Error::UnsafeArchive {
                    artifact: String::new(),
                    entry: e.path().display().to_string(),
                    why: "a link appeared in the extracted files".into(),
                });
            }
            if m.is_dir() {
                stack.push(e.path());
            }
        }
    }
    Ok(())
}

struct HashingReader<R> {
    inner: R,
    sha: Sha256,
    n: u64,
}

impl<R: Read> HashingReader<R> {
    fn new(inner: R) -> Self {
        HashingReader {
            inner,
            sha: Sha256::new(),
            n: 0,
        }
    }
    fn finish(self) -> (u64, String) {
        (self.n, hex(&self.sha.finalize()))
    }
}

impl<R: Read> Read for HashingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.sha.update(&buf[..n]);
        self.n += n as u64;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_names_are_checked() {
        assert_eq!(entry_parts(b"a/b/c.txt").unwrap(), ["a", "b", "c.txt"]);
        assert_eq!(entry_parts(b"./a//b/").unwrap(), ["a", "b"]);
        for bad in [
            &b"/etc/passwd"[..],
            b"../x",
            b"a/../../x",
            b"a\\..\\x",
            b"C:/Windows/x",
            b"a/file:stream",
            b"a/CON",
            b"a/nul.txt",
            b"a/com1",
            b"a/LPT9.log",
            b"a/x.",
            b"a/x ",
            b"a/x\0y",
            b"a/\x01",
            b"a/wild*",
            b"",
            b"\xff\xfe",
        ] {
            assert!(
                entry_parts(bad).is_err(),
                "{:?}",
                String::from_utf8_lossy(bad)
            );
        }
        // Not device names.
        assert!(entry_parts(b"a/CONSOLE.txt").is_ok());
        assert!(entry_parts(b"a/com0").is_ok());
        assert!(entry_parts(b"a/auxiliary").is_ok());
    }
}
