//! `content/pack.toml`: the content pack's identity, next to `problems/` and `learn/`.
//! Its id, version and date go into content.db (`content_meta`) and Settings → About
//! shows them. The content has its own version: it can change without an app release.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// The file's name, in the folder that holds `problems/`.
pub const PACK_FILE: &str = "pack.toml";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackInfo {
    /// `core` for the bundled pack: 1-32 lowercase letters, digits or `-`.
    pub id: String,
    /// `MAJOR.MINOR.PATCH`, with an optional `-pre.release` part.
    pub version: String,
    /// The content's release date, `YYYY-MM-DD`.
    pub date: String,
}

/// Where the pack file of the problems folder `problems` is.
pub fn pack_file_for(problems: &Path) -> PathBuf {
    problems.parent().unwrap_or(problems).join(PACK_FILE)
}

/// The pack file next to `problems`, checked; `None` when there is none.
pub fn read_pack_info(problems: &Path) -> Result<Option<PackInfo>, String> {
    let path = pack_file_for(problems);
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let info: PackInfo = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    check_pack_info(&info).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(Some(info))
}

/// The rules for each field.
pub fn check_pack_info(info: &PackInfo) -> Result<(), String> {
    let id_ok = (1..=32).contains(&info.id.len())
        && info.id.starts_with(|c: char| c.is_ascii_lowercase())
        && info
            .id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !id_ok {
        return Err(format!(
            "id {:?}: 1-32 lowercase letters, digits or '-'",
            info.id
        ));
    }
    if !version_ok(&info.version) {
        return Err(format!(
            "version {:?}: expected MAJOR.MINOR.PATCH, optionally with -pre.release",
            info.version
        ));
    }
    let date_ok = info.date.len() == 10
        && info
            .date
            .parse::<kb_core::Day>()
            .is_ok_and(|d| d.to_string() == info.date);
    if !date_ok {
        return Err(format!("date {:?}: expected a real YYYY-MM-DD", info.date));
    }
    Ok(())
}

fn version_ok(v: &str) -> bool {
    let (core, pre) = match v.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (v, None),
    };
    let number = |s: &str| {
        !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) && (s == "0" || !s.starts_with('0'))
    };
    let parts: Vec<&str> = core.split('.').collect();
    parts.len() == 3
        && parts.iter().all(|p| number(p))
        && pre.is_none_or(|p| {
            p.split('.').all(|id| {
                !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(id: &str, version: &str, date: &str) -> PackInfo {
        PackInfo {
            id: id.into(),
            version: version.into(),
            date: date.into(),
        }
    }

    #[test]
    fn checks_each_field() {
        assert!(check_pack_info(&info("core", "1.0.0", "2026-11-15")).is_ok());
        assert!(check_pack_info(&info("core-es", "2.10.3-beta.1", "2028-02-29")).is_ok());
        for bad in [
            info("", "1.0.0", "2026-11-15"),
            info("Core", "1.0.0", "2026-11-15"),
            info("core", "1.0", "2026-11-15"),
            info("core", "01.0.0", "2026-11-15"),
            info("core", "1.0.0-", "2026-11-15"),
            info("core", "1.0.0", "2026-02-30"),
            info("core", "1.0.0", "2027-02-29"),
            info("core", "1.0.0", "15/11/2026"),
            info("core", "1.0.0", "2026-W46-7"),
        ] {
            assert!(check_pack_info(&bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn the_repository_pack_file_is_valid() {
        let problems = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems");
        let info = read_pack_info(&problems)
            .unwrap()
            .expect("content/pack.toml");
        assert_eq!(info.id, "core");
    }
}
