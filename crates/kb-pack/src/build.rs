//! `kb-pack build`: compile the problem packages into content.db (kb-store). Refuses to
//! build unless every problem passes `kb-pack validate`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use kb_store::{PackMeta, PackProblem, PackSql, PackText, ProfileData, UiLang};

use crate::validate::{problem_dirs, read_problem_toml, validate_all};

pub struct BuildOptions {
    pub dir: PathBuf,
    pub out: PathBuf,
    pub pack_id: String,
    pub pack_version: String,
    /// `YYYY-MM-DD`, from `content/pack.toml`.
    pub pack_date: Option<String>,
}

/// Validate, read every package and write content.db. Returns the problem count.
pub fn build(opts: &BuildOptions) -> Result<usize, String> {
    if validate_all(&opts.dir) != ExitCode::SUCCESS {
        return Err("validation failed; nothing was built".into());
    }
    let dirs = problem_dirs(&opts.dir).map_err(|e| format!("{}: {e}", opts.dir.display()))?;
    let problems = dirs
        .iter()
        .map(|d| read_package(d))
        .collect::<Result<Vec<_>, _>>()?;
    let meta = PackMeta {
        pack_id: opts.pack_id.clone(),
        pack_version: opts.pack_version.clone(),
        pack_date: opts.pack_date.clone(),
    };
    // Learn topics (ADR 0029) from content/learn next to the problems, when present.
    let learn = crate::learn::read_learn(&crate::learn::learn_dir_for(&opts.dir), &opts.dir)
        .map_err(|e| format!("learn topics: {}", e.join("; ")))?;
    block_on(kb_store::build_with_learn(
        &opts.out,
        &meta,
        &problems,
        &learn.pack,
    ))
    .map_err(|e| e.to_string())?;
    Ok(problems.len())
}

pub(crate) fn block_on<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
        .block_on(f)
}

/// [`read_package`] for a pack's problem (expansion M11).
pub(crate) fn read_package_for_pack(dir: &Path) -> Result<PackProblem, String> {
    read_package(dir)
}

/// Read one (already validated) problem folder.
fn read_package(dir: &Path) -> Result<PackProblem, String> {
    let at = |e: String| format!("{}: {e}", dir.display());
    let meta = read_problem_toml(dir).map_err(at)?;
    let read =
        |name: &str| fs::read_to_string(dir.join(name)).map_err(|e| at(format!("{name}: {e}")));
    let read_opt = |name: &str| -> Result<Option<String>, String> {
        let p = dir.join(name);
        if p.is_file() {
            read(name).map(Some)
        } else {
            Ok(None)
        }
    };

    let mut texts = vec![PackText {
        lang: UiLang::En,
        title: meta.title.clone(),
        statement: read("statement.md")?,
        editorial: Some(read("editorial.md")?),
        hints: split_hints(&read("hints.md")?),
    }];
    texts.push(PackText {
        lang: UiLang::Es,
        title: meta.title_es.clone().unwrap_or_else(|| meta.title.clone()),
        statement: read("statement.es.md")?,
        editorial: read_opt("editorial.es.md")?,
        hints: read_opt("hints.es.md")?
            .as_deref()
            .map(split_hints)
            .unwrap_or_default(),
    });

    let problem = kb_judge::Problem::load(dir).map_err(|e| e.to_string())?;
    let mut solutions = Vec::new();
    let sql = match &problem.sql {
        // An SQL problem has one reference, its query (ADR 0031).
        Some(schema) => {
            solutions.push((kb_core::Language::Sql, read("solutions/ref.sql")?));
            Some(PackSql {
                schema_sql: schema.text.clone(),
                tables: schema.to_json(),
            })
        }
        None => {
            for lang in kb_judge::languages::SUPPORTED {
                solutions.push((lang, read(&format!("solutions/ref.{}", lang.extension()))?));
            }
            None
        }
    };
    let tests = problem.tests;

    Ok(PackProblem {
        id: meta.id,
        difficulty: meta.difficulty,
        tags: meta.tags,
        pattern: meta.pattern,
        rank: meta.rank,
        profile: if meta.profile {
            read_profile(dir).map_err(at)?
        } else {
            None
        },
        mode: meta.mode,
        // A program problem stores its synthetic signature (ADR 0028).
        signature: match &meta.signature {
            Some(sig) => serde_json::to_value(sig).map_err(|e| at(e.to_string()))?,
            None => serde_json::json!({
                "name": "main",
                "params": [{"name": kb_judge::problem::STDIN_PARAM, "type": "string"}],
                "returns": "string",
            }),
        },
        checker: meta.judge.checker,
        time_limit_ms: meta.judge.time_limit_ms,
        memory_mb: meta.judge.memory_mb,
        texts,
        tests,
        solutions,
        // The app runs it on custom test inputs (ADR 0016).
        validator: fs::read_to_string(dir.join("validate.py")).ok(),
        sql,
        parent: meta.parent,
        // Set for pack problems (expansion M11).
        pack: None,
        // The stress lab (expansion M8) generates inputs with gen.py.
        generator: fs::read_to_string(dir.join("gen.py")).ok(),
        // Fix the bug (expansion M6): the buggy code becomes the starter.
        fix: match meta.kind.as_deref() {
            Some("fix") => Some(
                kb_judge::languages::SUPPORTED
                    .into_iter()
                    .filter_map(|lang| {
                        fs::read_to_string(
                            dir.join("solutions")
                                .join(format!("buggy.{}", lang.extension())),
                        )
                        .ok()
                        .map(|code| (lang, code))
                    })
                    .collect(),
            ),
            _ => None,
        },
    })
}

/// The cached profile inputs, verified against their hashes, and the reference times
/// (ADR 0024). Without a complete cache the problem is packed without a profile.
fn read_profile(dir: &Path) -> Result<Option<ProfileData>, String> {
    let reference = crate::profile::read_reference(dir)?;
    let Some(inputs) = crate::profile::cached_inputs(dir, &reference)? else {
        eprintln!(
            "warning: {}: no profile inputs cached, packed without a complexity profile \
             (run kb-pack profile --inputs)",
            dir.display()
        );
        return Ok(None);
    };
    let mut times = Vec::new();
    for lang in crate::profile::languages() {
        let t = reference
            .times_us
            .get(lang.id())
            .ok_or_else(|| format!("profile/reference.json: no {} times", lang.id()))?;
        times.push((lang, t.clone()));
    }
    Ok(Some(ProfileData {
        inputs,
        reference: times,
    }))
}

/// Hint tiers are separated by lines containing only `---`.
fn split_hints(src: &str) -> Vec<String> {
    let mut tiers = vec![String::new()];
    for line in src.lines() {
        if line.trim() == "---" {
            tiers.push(String::new());
        } else {
            let t = tiers.last_mut().expect("never empty");
            t.push_str(line);
            t.push('\n');
        }
    }
    tiers
        .into_iter()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::tests::copy_dir;
    use kb_core::{Language, TestKind};
    use kb_store::{ContentDb, ProblemFilter};
    use sqlx::Row;

    fn repo() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("kbp-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn opts(dir: PathBuf, out: PathBuf) -> BuildOptions {
        BuildOptions {
            dir,
            out,
            pack_id: "core".into(),
            pack_version: "0.1.0".into(),
            pack_date: Some("2026-11-15".into()),
        }
    }

    #[test]
    fn splits_hint_tiers() {
        assert_eq!(split_hints("a\nb\n---\nc\n---\n\n"), ["a\nb", "c"]);
        assert_eq!(split_hints("only one"), ["only one"]);
    }

    #[test]
    fn builds_two_sum() {
        let tmp = temp("two-sum");
        let out = tmp.join("content.db");
        let n = build(&opts(repo().join("content/problems"), out.clone())).unwrap();
        assert!(n >= 1);
        block_on(async {
            let db = ContentDb::open(&out).await.unwrap();
            let meta = db.meta().await.unwrap();
            assert_eq!(
                (meta.pack_id.as_str(), meta.pack_version.as_str()),
                ("core", "0.1.0")
            );

            let en = db.get_problem("two-sum", UiLang::En).await.unwrap();
            assert_eq!(en.title, "Two Sum");
            assert_eq!(en.hints.len(), 3);
            assert!(en.editorial.is_some());
            assert_eq!(en.public_tests.len(), 3);
            assert_eq!(en.signature["name"], "twoSum");
            assert_eq!(en.signature["params"][0]["type"], "int[]");
            let es = db.get_problem("two-sum", UiLang::Es).await.unwrap();
            assert_eq!(es.title, "Suma de dos");
            assert!(es.statement.contains("índices"));
            assert_eq!(es.hints.len(), 3);
            assert_ne!(es.hints, en.hints); // hints.es.md
            assert!(en.has_validator);

            let jd = db.judge_data("two-sum").await.unwrap();
            let hidden = jd
                .tests
                .iter()
                .filter(|t| t.kind == TestKind::Hidden)
                .count();
            assert_eq!((jd.tests.len(), hidden), (3 + hidden, hidden));
            assert!(hidden >= 15, "{hidden} hidden tests");
            assert!(
                jd.validator
                    .as_deref()
                    .is_some_and(|v| v.contains("def validate("))
            );
            assert_eq!(jd.checker, "unordered");
            let disk = kb_judge::Problem::load(&repo().join("content/problems/two-sum")).unwrap();
            assert_eq!(jd.tests, disk.tests);
            for lang in kb_judge::languages::SUPPORTED {
                assert!(
                    db.reference_solution("two-sum", lang)
                        .await
                        .unwrap()
                        .is_some()
                );
            }
            assert!(
                db.reference_solution("two-sum", Language::JavaScript)
                    .await
                    .unwrap()
                    .is_none()
            );

            for q in ["two sum", "TWO", "hash-map", "índices", "indices", "suma"] {
                let hits = db
                    .list_problems(&ProblemFilter {
                        query: Some(q.into()),
                        ..Default::default()
                    })
                    .await
                    .unwrap();
                assert!(hits.iter().any(|p| p.id == "two-sum"), "query {q:?}");
            }
            db.close().await;
        });
    }

    /// Every row of every table (FTS shadow tables included) and the schema, as text.
    async fn dump(path: &Path) -> Vec<String> {
        use sqlx::Connection;
        let opts = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(path)
            .read_only(true);
        let mut conn = sqlx::SqliteConnection::connect_with(&opts).await.unwrap();
        let mut out = Vec::new();
        let schema = sqlx::query(
            "SELECT type, name, COALESCE(sql, '') AS sql FROM sqlite_master ORDER BY name",
        )
        .fetch_all(&mut conn)
        .await
        .unwrap();
        let mut tables = Vec::new();
        for r in &schema {
            let (ty, name, sql): (String, String, String) =
                (r.get("type"), r.get("name"), r.get("sql"));
            if ty == "table" && !sql.starts_with("CREATE VIRTUAL") {
                tables.push(name.clone());
            }
            out.push(format!("schema {ty} {name} {sql}"));
        }
        for t in tables {
            let cols: Vec<String> = sqlx::query(&format!("PRAGMA table_info(\"{t}\")"))
                .fetch_all(&mut conn)
                .await
                .unwrap()
                .iter()
                .map(|r| format!("quote(\"{}\")", r.get::<String, _>("name")))
                .collect();
            let rows: Vec<String> =
                sqlx::query_scalar(&format!("SELECT {} FROM \"{t}\"", cols.join(" || '|' || ")))
                    .fetch_all(&mut conn)
                    .await
                    .unwrap();
            let mut rows: Vec<String> = rows
                .into_iter()
                .filter(|r| !(t == "content_meta" && r.starts_with("'built_at'|")))
                .map(|r| format!("{t}: {r}"))
                .collect();
            rows.sort();
            out.extend(rows);
        }
        out
    }

    #[test]
    fn rebuild_is_identical_except_build_time() {
        let tmp = temp("rebuild");
        let (a, b) = (tmp.join("a.db"), tmp.join("b.db"));
        build(&opts(repo().join("content/problems"), a.clone())).unwrap();
        build(&opts(repo().join("content/problems"), b.clone())).unwrap();
        let (da, db) = block_on(async { (dump(&a).await, dump(&b).await) });
        assert!(da.iter().any(|r| r.starts_with("problem_fts_data: ")));
        assert!(da.iter().any(|r| r.starts_with("tests: ")));
        assert_eq!(da, db);
    }

    #[test]
    fn refuses_an_invalid_problem() {
        let tmp = temp("invalid");
        let problems = tmp.join("problems");
        copy_dir(
            &repo().join("content/problems/two-sum"),
            &problems.join("two-sum"),
        );
        fs::remove_dir_all(problems.join("two-sum/tests/hidden")).unwrap();
        let out = tmp.join("content.db");
        let err = build(&opts(problems, out.clone())).unwrap_err();
        assert!(err.contains("validation failed"), "{err}");
        assert!(!out.exists());
    }
}
