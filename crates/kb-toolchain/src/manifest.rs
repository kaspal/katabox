//! The compiled-in manifest: artifacts (URL, size, SHA-256), components (one folder each)
//! and languages (what the user picks). See `manifest.toml`.

use std::sync::OnceLock;

use kb_core::Language;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{Error, Result};

const MANIFEST_TOML: &str = include_str!("../manifest.toml");

/// Where the app runs. Artifacts and components are per platform (`any` fits all).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    WindowsX86_64,
    LinuxX86_64,
    LinuxAarch64,
}

impl Platform {
    /// The platform this binary was built for, if KataBox supports it.
    pub fn current() -> Option<Platform> {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("windows", "x86_64") => Some(Platform::WindowsX86_64),
            ("linux", "x86_64") => Some(Platform::LinuxX86_64),
            ("linux", "aarch64") => Some(Platform::LinuxAarch64),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Platform::WindowsX86_64 => "windows-x86_64",
            Platform::LinuxX86_64 => "linux-x86_64",
            Platform::LinuxAarch64 => "linux-aarch64",
        }
    }

    pub const ALL: [Platform; 3] = [
        Platform::WindowsX86_64,
        Platform::LinuxX86_64,
        Platform::LinuxAarch64,
    ];

    fn matches(self, p: &str) -> bool {
        p == "any" || p == self.id()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Format {
    #[serde(rename = "zip")]
    Zip,
    #[serde(rename = "tar.gz")]
    TarGz,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub id: String,
    pub platform: String,
    /// Mirrors of the same file (same size and hash), tried in order.
    pub urls: Vec<String>,
    pub size: u64,
    pub sha256: String,
    pub format: Format,
    /// npm's SRI hash (`sha512-…`), checked in addition to `sha256`.
    #[serde(default)]
    pub integrity: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub artifact: String,
    /// Extract this folder of the archive (`""` = all of it); other entries are ignored.
    #[serde(default)]
    pub subtree: Option<String>,
    /// Paths (relative to `subtree`) left out.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Symlinks (relative to `subtree`) copied as regular files.
    #[serde(default)]
    pub links: Vec<String>,
    /// Symlinks left out; an entry ending in `/` is a prefix.
    #[serde(default)]
    pub skip_links: Vec<String>,
    /// Instead of a subtree: exactly this archive member, placed at the component root
    /// under its file name, with its own hash.
    #[serde(default)]
    pub member: Option<String>,
    #[serde(default)]
    pub member_sha256: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Component {
    pub id: String,
    pub platform: String,
    /// Folder under the sandbox root, `/`-separated.
    pub dest: String,
    /// A file that must exist in a complete install.
    pub check: String,
    /// Bytes on disk once installed (for the size shown and the space check).
    pub installed_size: u64,
    #[serde(rename = "source")]
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguageUnit {
    pub id: String,
    /// What the judge runs.
    pub runtime: Vec<String>,
    /// What the language server needs.
    pub servers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    pub hosts: Vec<String>,
    #[serde(rename = "artifact")]
    pub artifacts: Vec<Artifact>,
    #[serde(rename = "component")]
    pub components: Vec<Component>,
    #[serde(rename = "language")]
    pub languages: Vec<LanguageUnit>,
}

impl Manifest {
    /// The manifest compiled into this binary (validated by the crate's tests).
    pub fn builtin() -> &'static Manifest {
        static M: OnceLock<Manifest> = OnceLock::new();
        M.get_or_init(|| Manifest::parse(MANIFEST_TOML).expect("built-in manifest is valid"))
    }

    pub fn parse(text: &str) -> Result<Manifest> {
        let m: Manifest = toml::from_str(text).map_err(|e| Error::Manifest(e.to_string()))?;
        m.validate(false)?;
        Ok(m)
    }

    /// Like [`Manifest::parse`] but URLs may name a port: the local HTTPS test server.
    /// The app only uses [`Manifest::builtin`].
    #[doc(hidden)]
    pub fn parse_allowing_any_port(text: &str) -> Result<Manifest> {
        let m: Manifest = toml::from_str(text).map_err(|e| Error::Manifest(e.to_string()))?;
        m.validate(true)?;
        Ok(m)
    }

    pub fn artifact(&self, id: &str, platform: Platform) -> Option<&Artifact> {
        self.artifacts
            .iter()
            .find(|a| a.id == id && platform.matches(&a.platform))
    }

    pub fn component(&self, id: &str, platform: Platform) -> Option<&Component> {
        self.components
            .iter()
            .find(|c| c.id == id && platform.matches(&c.platform))
    }

    /// The unit for a language. JavaScript is part of the TypeScript unit.
    pub fn language(&self, language: Language) -> Option<&LanguageUnit> {
        let id = unit_id(language);
        self.languages.iter().find(|l| l.id == id)
    }

    /// Component ids of a language (runtime first), or `None` when some component has no
    /// build for `platform`.
    pub fn language_components(
        &self,
        language: Language,
        platform: Platform,
        runtimes_only: bool,
    ) -> Option<Vec<&Component>> {
        let unit = self.language(language)?;
        let ids = unit.runtime.iter().chain(if runtimes_only {
            [].iter()
        } else {
            unit.servers.iter()
        });
        let mut out: Vec<&Component> = Vec::new();
        for id in ids {
            let c = self.component(id, platform)?;
            if !out.iter().any(|x| x.id == c.id) {
                out.push(c);
            }
        }
        // A language is available only if everything it needs is, servers included.
        for id in unit.runtime.iter().chain(unit.servers.iter()) {
            self.component(id, platform)?;
        }
        Some(out)
    }

    /// Identifies what a component's folder contains: its sources, their hashes and the
    /// layout rules. A receipt is valid only for the digest it was written with.
    pub fn digest(&self, c: &Component, platform: Platform) -> String {
        let mut h = Sha256::new();
        let mut put = |s: &str| {
            h.update((s.len() as u64).to_le_bytes());
            h.update(s.as_bytes());
        };
        put(&c.id);
        put(&c.dest);
        put(&c.check);
        for s in &c.sources {
            put(&s.artifact);
            if let Some(a) = self.artifact(&s.artifact, platform) {
                put(&a.sha256);
            }
            put(s.subtree.as_deref().unwrap_or("\u{0}"));
            put(s.member.as_deref().unwrap_or("\u{0}"));
            put(s.member_sha256.as_deref().unwrap_or("\u{0}"));
            for list in [&s.exclude, &s.links, &s.skip_links] {
                put(&list.len().to_string());
                for x in list {
                    put(x);
                }
            }
        }
        hex(&h.finalize())
    }

    fn validate(&self, any_port: bool) -> Result<()> {
        let bad = |why: String| Err(Error::Manifest(why));
        if self.schema != 1 {
            return bad(format!("unsupported schema {}", self.schema));
        }
        for a in &self.artifacts {
            if a.urls.is_empty() {
                return bad(format!("{}: no urls", a.id));
            }
            for u in &a.urls {
                check_url(u, &self.hosts, any_port)
                    .map_err(|why| Error::Manifest(format!("{}: {why}", a.id)))?;
            }
            if !is_sha256(&a.sha256) {
                return bad(format!("{}: sha256 must be 64 lowercase hex digits", a.id));
            }
            if a.size == 0 {
                return bad(format!("{}: size must be set", a.id));
            }
            if !(a.platform == "any" || Platform::ALL.iter().any(|p| p.id() == a.platform)) {
                return bad(format!("{}: unknown platform {}", a.id, a.platform));
            }
        }
        for c in &self.components {
            crate::layout::check_dest(&c.dest)
                .map_err(|why| Error::Manifest(format!("{}: {why}", c.id)))?;
            for p in Platform::ALL {
                if !p.matches(&c.platform) {
                    continue;
                }
                for s in &c.sources {
                    if self.artifact(&s.artifact, p).is_none() {
                        return bad(format!("{} ({}): no artifact {}", c.id, p.id(), s.artifact));
                    }
                    match (&s.member, &s.member_sha256, &s.subtree) {
                        (Some(_), Some(h), None) if is_sha256(h) => {}
                        (None, None, Some(_)) => {}
                        _ => {
                            return bad(format!(
                                "{}: a source has either `subtree`, or `member` with `member_sha256`",
                                c.id
                            ));
                        }
                    }
                }
            }
        }
        for l in &self.languages {
            if l.runtime.is_empty() {
                return bad(format!("language {}: no runtime", l.id));
            }
            for id in l.runtime.iter().chain(&l.servers) {
                if !self.components.iter().any(|c| &c.id == id) {
                    return bad(format!("language {}: unknown component {id}", l.id));
                }
            }
        }
        Ok(())
    }
}

/// The manifest unit id for a language.
pub fn unit_id(language: Language) -> &'static str {
    match language {
        Language::JavaScript | Language::TypeScript => "typescript",
        other => other.id(),
    }
}

/// Languages the user picks from (JavaScript comes with TypeScript).
pub const UNITS: [Language; 4] = [
    Language::Python,
    Language::TypeScript,
    Language::Php,
    Language::Rust,
];

/// A download URL must be `https://<allowed host>/…`: port 443 only, no user info.
pub(crate) fn check_url(
    url: &str,
    hosts: &[String],
    any_port: bool,
) -> std::result::Result<(), String> {
    let u = reqwest::Url::parse(url).map_err(|e| format!("{url}: {e}"))?;
    if u.scheme() != "https" {
        return Err(format!("{url}: not https"));
    }
    if !u.username().is_empty() || u.password().is_some() {
        return Err(format!("{url}: user info is not allowed"));
    }
    if !any_port && u.port().is_some_and(|p| p != 443) {
        return Err(format!("{url}: only port 443"));
    }
    match u.host_str() {
        Some(h) if hosts.iter().any(|a| a.eq_ignore_ascii_case(h)) => Ok(()),
        _ => Err(format!("{url}: host not on the allow-list")),
    }
}

pub(crate) fn is_sha256(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_manifest_is_valid_for_every_platform() {
        let m = Manifest::builtin();
        for p in Platform::ALL {
            // Python and JS/TS everywhere; PHP and Rust on Windows only (ADR 0009).
            for l in [Language::Python, Language::TypeScript, Language::JavaScript] {
                assert!(
                    m.language_components(l, p, false).is_some(),
                    "{l} on {}",
                    p.id()
                );
            }
            let win = p == Platform::WindowsX86_64;
            for l in [Language::Php, Language::Rust] {
                assert_eq!(
                    m.language_components(l, p, false).is_some(),
                    win,
                    "{l} on {}",
                    p.id()
                );
            }
        }
        let comps = m
            .language_components(Language::Rust, Platform::WindowsX86_64, false)
            .unwrap();
        let ids: Vec<&str> = comps.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(ids, ["rust", "rust-src", "rust-analyzer"]);
        let comps = m
            .language_components(Language::Python, Platform::LinuxX86_64, true)
            .unwrap();
        assert_eq!(comps.len(), 1);
    }

    #[test]
    fn urls_must_be_https_on_allowed_hosts() {
        let hosts = vec!["nodejs.org".to_string()];
        assert!(check_url("https://nodejs.org/dist/x.zip", &hosts, false).is_ok());
        assert!(check_url("https://NODEJS.org/dist/x.zip", &hosts, false).is_ok());
        for bad in [
            "http://nodejs.org/x.zip",
            "https://nodejs.org:8443/x.zip",
            "https://user:pw@nodejs.org/x.zip",
            "https://nodejs.org.evil.example/x.zip",
            "https://evil.example/nodejs.org/x.zip",
            "ftp://nodejs.org/x.zip",
        ] {
            assert!(check_url(bad, &hosts, false).is_err(), "{bad}");
        }
        let bad = MANIFEST_TOML.replace(
            "https://nodejs.org/dist/v26.13.0/node-v26.13.0-win-x64.zip",
            "http://nodejs.org/x.zip",
        );
        assert!(Manifest::parse(&bad).is_err());
        let bad = MANIFEST_TOML.replace("\"nodejs.org\",", "");
        assert!(Manifest::parse(&bad).is_err());
    }

    #[test]
    fn digests_change_with_pins_and_layout() {
        let m = Manifest::builtin();
        let p = Platform::WindowsX86_64;
        let c = m.component("python", p).unwrap();
        let d = m.digest(c, p);
        assert_eq!(d, m.digest(c, p));
        let mut c2 = c.clone();
        c2.sources[0].subtree = Some("other".into());
        assert_ne!(d, m.digest(&c2, p));
        let mut m2 = m.clone();
        m2.artifacts
            .iter_mut()
            .find(|a| a.id == "python-windows-x86_64")
            .unwrap()
            .sha256 = "0".repeat(64);
        assert_ne!(d, m2.digest(c, p));
        // Other components are unaffected by a Python pin change.
        let n = m.component("node", p).unwrap();
        assert_eq!(m.digest(n, p), m2.digest(n, p));
    }

    /// Review item: the npm packages installed are exactly the lockfile's non-optional
    /// packages (same version URL and integrity), so a new dependency can never be
    /// silently missing at runtime, and nothing outside the lockfile is installed.
    #[test]
    fn npm_artifacts_match_the_lockfile_exactly() {
        let lock: serde_json::Value = serde_json::from_str(include_str!(
            "../../../scripts/language-servers/package-lock.json"
        ))
        .unwrap();
        let mut from_lock = std::collections::BTreeMap::new();
        for (key, pkg) in lock["packages"].as_object().unwrap() {
            if key.is_empty() || pkg["optional"].as_bool() == Some(true) {
                continue;
            }
            let name = key
                .strip_prefix("node_modules/")
                .expect("a top-level node_modules entry");
            assert!(
                !name.contains("/node_modules/"),
                "nested dependency {key}: the manifest must install it"
            );
            from_lock.insert(
                name.to_string(),
                (
                    pkg["resolved"].as_str().unwrap().to_string(),
                    pkg["integrity"].as_str().unwrap().to_string(),
                ),
            );
        }
        let m = Manifest::builtin();
        let mut from_manifest = std::collections::BTreeMap::new();
        for c in m
            .components
            .iter()
            .filter(|c| c.dest.starts_with("tools/language-servers/node_modules/"))
        {
            let name = c.dest.rsplit('/').next().unwrap().to_string();
            assert_eq!(c.sources.len(), 1, "{name}");
            let a = m
                .artifact(&c.sources[0].artifact, Platform::WindowsX86_64)
                .unwrap();
            assert_eq!(a.urls.len(), 1, "{name}");
            from_manifest.insert(
                name,
                (
                    a.urls[0].clone(),
                    a.integrity.clone().expect("npm artifacts carry integrity"),
                ),
            );
        }
        assert_eq!(from_lock, from_manifest);
        // Every npm artifact is installed by some component.
        for a in m
            .artifacts
            .iter()
            .filter(|a| a.urls[0].starts_with("https://registry.npmjs.org/"))
        {
            assert!(
                m.components
                    .iter()
                    .any(|c| c.sources.iter().any(|s| s.artifact == a.id)),
                "{}",
                a.id
            );
        }
    }

    #[test]
    fn hashes_and_sizes_are_well_formed() {
        let m = Manifest::builtin();
        assert!(
            m.artifacts
                .iter()
                .all(|a| is_sha256(&a.sha256) && a.size > 0)
        );
        assert!(m.components.iter().all(|c| c.installed_size > 0));
    }
}
