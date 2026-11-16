//! The Learn module's content (ADR 0029): subjects, topics with their theory, samples and
//! quiz, built into content.db by kb-pack and read by the app. The tracers that animate
//! the topics are TypeScript in the app, never content.

use std::collections::HashSet;
use std::str::FromStr;

use kb_core::Language;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqliteConnection};

use crate::content::{ContentDb, UiLang, fts_query};
use crate::{Result, StoreError};

/// Every subject and topic to write, already read and validated (kb-pack does that).
#[derive(Debug, Clone, Default)]
pub struct LearnPack {
    pub subjects: Vec<PackSubject>,
    pub topics: Vec<PackTopic>,
    /// Learning paths (Phase 6): ordered topic lists shown on Learn home.
    pub paths: Vec<PackPath>,
}

/// A learning path as `content/learn/paths.toml` holds it.
#[derive(Debug, Clone)]
pub struct PackPath {
    pub id: String,
    pub ord: u32,
    pub title_en: String,
    pub title_es: String,
    pub blurb_en: String,
    pub blurb_es: String,
    /// Topic ids, in the order to study them.
    pub topics: Vec<String>,
}

/// A learning path in one UI language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LearnPath {
    pub id: String,
    pub title: String,
    pub blurb: String,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PackSubject {
    pub id: String,
    pub ord: u32,
    pub name_en: String,
    pub name_es: String,
    pub blurb_en: String,
    pub blurb_es: String,
    /// A name from the UI's fixed icon map.
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct PackTopic {
    pub id: String,
    pub subject: String,
    pub ord: u32,
    pub title_en: String,
    pub title_es: String,
    pub tags: Vec<String>,
    pub complexity: LearnComplexity,
    /// Linked catalogue problems, in display order.
    pub problems: Vec<String>,
    pub presets: Vec<String>,
    pub input: LearnInput,
    pub default_input: Vec<i64>,
    pub theory_en: String,
    pub theory_es: String,
    /// Raw sources, `@step` markers included.
    pub samples: Vec<(Language, String)>,
    pub quiz: Vec<PackQuizQuestion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LearnComplexity {
    pub best: String,
    pub average: String,
    pub worst: String,
    pub space: String,
    pub stable: bool,
    pub in_place: bool,
}

/// The limits of "your own" input, also the tracer's cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LearnInput {
    pub min_len: u32,
    pub max_len: u32,
    pub min: i32,
    pub max: i32,
}

/// One quiz question in both languages, as `quiz.json` holds it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackQuizQuestion {
    pub prompt_en: String,
    pub prompt_es: String,
    pub options: Vec<PackQuizOption>,
    /// 0-based index into `options`.
    pub answer: u32,
    pub explain_en: String,
    pub explain_es: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackQuizOption {
    pub en: String,
    pub es: String,
}

/// A subject in the catalogue, with its topics (possibly none yet: "Coming soon").
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LearnSubject {
    pub id: String,
    pub name: String,
    pub blurb: String,
    pub icon: String,
    pub topics: Vec<LearnTopicSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LearnTopicSummary {
    pub id: String,
    pub subject: String,
    pub title: String,
    pub tags: Vec<String>,
    /// Linked catalogue problems: a problem page links back here ("See it animated").
    pub problems: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LearnSample {
    pub language: Language,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct QuizQuestion {
    pub prompt: String,
    pub options: Vec<String>,
    pub answer: u32,
    pub explain: String,
}

/// Everything the topic page shows, in one UI language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LearnTopic {
    pub id: String,
    pub subject: String,
    pub subject_name: String,
    pub title: String,
    pub tags: Vec<String>,
    pub complexity: LearnComplexity,
    pub problems: Vec<String>,
    pub presets: Vec<String>,
    pub input: LearnInput,
    pub default_input: Vec<i64>,
    pub theory: String,
    pub samples: Vec<LearnSample>,
    pub quiz: Vec<QuizQuestion>,
}

pub(crate) fn check_learn(pack: &LearnPack) -> Result<()> {
    let bad = |m: String| Err(StoreError::InvalidPack(m));
    let mut subjects = HashSet::new();
    for s in &pack.subjects {
        if !subjects.insert(s.id.as_str()) {
            return bad(format!("duplicate subject {}", s.id));
        }
    }
    let mut topics = HashSet::new();
    let mut places = HashSet::new();
    for t in &pack.topics {
        if !topics.insert(t.id.as_str()) {
            return bad(format!("duplicate topic {}", t.id));
        }
        if !subjects.contains(t.subject.as_str()) {
            return bad(format!("{}: unknown subject {}", t.id, t.subject));
        }
        if !places.insert((t.subject.as_str(), t.ord)) {
            return bad(format!(
                "{}: order {} used twice in {}",
                t.id, t.ord, t.subject
            ));
        }
    }
    let mut paths = HashSet::new();
    for p in &pack.paths {
        if !paths.insert(p.id.as_str()) {
            return bad(format!("duplicate path {}", p.id));
        }
        if p.topics.is_empty() {
            return bad(format!("path {} has no topics", p.id));
        }
        if let Some(t) = p.topics.iter().find(|t| !topics.contains(t.as_str())) {
            return bad(format!("path {}: unknown topic {t}", p.id));
        }
    }
    Ok(())
}

pub(crate) async fn insert_learn(conn: &mut SqliteConnection, pack: &LearnPack) -> Result<()> {
    let mut subjects: Vec<&PackSubject> = pack.subjects.iter().collect();
    subjects.sort_by(|a, b| a.id.cmp(&b.id));
    for s in subjects {
        sqlx::query(
            "INSERT INTO learn_subjects (id, ord, name_en, name_es, blurb_en, blurb_es, icon)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&s.id)
        .bind(s.ord)
        .bind(&s.name_en)
        .bind(&s.name_es)
        .bind(&s.blurb_en)
        .bind(&s.blurb_es)
        .bind(&s.icon)
        .execute(&mut *conn)
        .await?;
    }
    for p in &pack.paths {
        sqlx::query(
            "INSERT INTO learn_paths (id, ord, title_en, title_es, blurb_en, blurb_es, topics)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&p.id)
        .bind(p.ord)
        .bind(&p.title_en)
        .bind(&p.title_es)
        .bind(&p.blurb_en)
        .bind(&p.blurb_es)
        .bind(serde_json::to_string(&p.topics)?)
        .execute(&mut *conn)
        .await?;
    }
    let mut topics: Vec<&PackTopic> = pack.topics.iter().collect();
    topics.sort_by(|a, b| a.id.cmp(&b.id));
    for t in topics {
        sqlx::query(
            "INSERT INTO learn_topics (id, subject, ord, title_en, title_es, tags, complexity,
                                       problems, presets, input, default_input)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&t.id)
        .bind(&t.subject)
        .bind(t.ord)
        .bind(&t.title_en)
        .bind(&t.title_es)
        .bind(serde_json::to_string(&t.tags)?)
        .bind(serde_json::to_string(&t.complexity)?)
        .bind(serde_json::to_string(&t.problems)?)
        .bind(serde_json::to_string(&t.presets)?)
        .bind(serde_json::to_string(&t.input)?)
        .bind(serde_json::to_string(&t.default_input)?)
        .execute(&mut *conn)
        .await?;
        let tags = t.tags.join(" ");
        for (lang, title, theory) in [
            (UiLang::En, &t.title_en, &t.theory_en),
            (UiLang::Es, &t.title_es, &t.theory_es),
        ] {
            sqlx::query("INSERT INTO learn_texts (topic_id, lang, theory) VALUES (?, ?, ?)")
                .bind(&t.id)
                .bind(lang.code())
                .bind(theory)
                .execute(&mut *conn)
                .await?;
            sqlx::query(
                "INSERT INTO learn_fts (topic_id, lang, title, tags, theory)
                 VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&t.id)
            .bind(lang.code())
            .bind(title)
            .bind(&tags)
            .bind(theory)
            .execute(&mut *conn)
            .await?;
        }
        let mut samples: Vec<&(Language, String)> = t.samples.iter().collect();
        samples.sort_by_key(|(l, _)| l.id());
        for (language, source) in samples {
            sqlx::query("INSERT INTO learn_samples (topic_id, language, source) VALUES (?, ?, ?)")
                .bind(&t.id)
                .bind(language.id())
                .bind(source)
                .execute(&mut *conn)
                .await?;
        }
        sqlx::query("INSERT INTO learn_quiz (topic_id, json) VALUES (?, ?)")
            .bind(&t.id)
            .bind(serde_json::to_string(&t.quiz)?)
            .execute(&mut *conn)
            .await?;
    }
    Ok(())
}

fn pick(lang: UiLang, en: String, es: String) -> String {
    match lang {
        UiLang::Es if !es.is_empty() => es,
        _ => en,
    }
}

fn json<T: serde::de::DeserializeOwned>(r: &SqliteRow, col: &str) -> Result<T> {
    Ok(serde_json::from_str(&r.try_get::<String, _>(col)?)?)
}

fn summary(r: &SqliteRow, lang: UiLang) -> Result<LearnTopicSummary> {
    Ok(LearnTopicSummary {
        id: r.try_get("id")?,
        subject: r.try_get("subject")?,
        title: pick(lang, r.try_get("title_en")?, r.try_get("title_es")?),
        tags: json(r, "tags")?,
        problems: json(r, "problems")?,
    })
}

impl ContentDb {
    /// Every subject in order, each with its topics in order.
    pub async fn learn_catalog(&self, lang: UiLang) -> Result<Vec<LearnSubject>> {
        let subjects = sqlx::query("SELECT * FROM learn_subjects ORDER BY ord, id")
            .fetch_all(self.pool())
            .await?;
        let topics = sqlx::query("SELECT * FROM learn_topics ORDER BY subject, ord, id")
            .fetch_all(self.pool())
            .await?;
        let topics = topics
            .iter()
            .map(|r| summary(r, lang))
            .collect::<Result<Vec<_>>>()?;
        subjects
            .iter()
            .map(|r| {
                let id: String = r.try_get("id")?;
                Ok(LearnSubject {
                    name: pick(lang, r.try_get("name_en")?, r.try_get("name_es")?),
                    blurb: pick(lang, r.try_get("blurb_en")?, r.try_get("blurb_es")?),
                    icon: r.try_get("icon")?,
                    topics: topics.iter().filter(|t| t.subject == id).cloned().collect(),
                    id,
                })
            })
            .collect()
    }

    /// The learning paths in order (Phase 6), in `lang`.
    pub async fn learn_paths(&self, lang: UiLang) -> Result<Vec<LearnPath>> {
        let rows = sqlx::query("SELECT * FROM learn_paths ORDER BY ord, id")
            .fetch_all(self.pool())
            .await?;
        rows.iter()
            .map(|r| {
                Ok(LearnPath {
                    id: r.try_get("id")?,
                    title: pick(lang, r.try_get("title_en")?, r.try_get("title_es")?),
                    blurb: pick(lang, r.try_get("blurb_en")?, r.try_get("blurb_es")?),
                    topics: json(r, "topics")?,
                })
            })
            .collect()
    }

    /// One topic page in `lang`.
    pub async fn learn_topic(&self, id: &str, lang: UiLang) -> Result<LearnTopic> {
        let t = sqlx::query(
            "SELECT t.*, s.name_en AS s_en, s.name_es AS s_es
             FROM learn_topics t JOIN learn_subjects s ON s.id = t.subject
             WHERE t.id = ?",
        )
        .bind(id)
        .fetch_optional(self.pool())
        .await?
        .ok_or_else(|| StoreError::NotFound(format!("topic {id}")))?;
        let texts = sqlx::query("SELECT lang, theory FROM learn_texts WHERE topic_id = ?")
            .bind(id)
            .fetch_all(self.pool())
            .await?;
        let theory_in = |want: &str| {
            texts
                .iter()
                .find(|r| r.get::<String, _>("lang") == want)
                .map(|r| r.get::<String, _>("theory"))
        };
        let theory = theory_in(lang.code())
            .or_else(|| theory_in("en"))
            .ok_or_else(|| StoreError::Corrupt(format!("topic {id} has no theory")))?;
        let samples = sqlx::query(
            "SELECT language, source FROM learn_samples WHERE topic_id = ? ORDER BY language",
        )
        .bind(id)
        .fetch_all(self.pool())
        .await?
        .iter()
        .filter_map(|r| {
            let language = Language::from_str(&r.get::<String, _>("language")).ok()?;
            Some(LearnSample {
                language,
                source: r.get("source"),
            })
        })
        .collect();
        let quiz: String = sqlx::query_scalar("SELECT json FROM learn_quiz WHERE topic_id = ?")
            .bind(id)
            .fetch_optional(self.pool())
            .await?
            .unwrap_or_else(|| "[]".into());
        let quiz: Vec<PackQuizQuestion> = serde_json::from_str(&quiz)?;
        let es = lang == UiLang::Es;
        let quiz = quiz
            .into_iter()
            .map(|q| QuizQuestion {
                prompt: if es { q.prompt_es } else { q.prompt_en },
                options: q
                    .options
                    .into_iter()
                    .map(|o| if es { o.es } else { o.en })
                    .collect(),
                answer: q.answer,
                explain: if es { q.explain_es } else { q.explain_en },
            })
            .collect();
        Ok(LearnTopic {
            id: t.try_get("id")?,
            subject: t.try_get("subject")?,
            subject_name: pick(lang, t.try_get("s_en")?, t.try_get("s_es")?),
            title: pick(lang, t.try_get("title_en")?, t.try_get("title_es")?),
            tags: json(&t, "tags")?,
            complexity: json(&t, "complexity")?,
            problems: json(&t, "problems")?,
            presets: json(&t, "presets")?,
            input: json(&t, "input")?,
            default_input: json(&t, "default_input")?,
            theory,
            samples,
            quiz,
        })
    }

    /// Topics whose title, tags or theory match `query` in any language, in catalogue
    /// order, titled in `lang`. Text with no words matches nothing.
    pub async fn search_learn(&self, query: &str, lang: UiLang) -> Result<Vec<LearnTopicSummary>> {
        let Some(fts) = fts_query(query) else {
            return Ok(Vec::new());
        };
        let rows = sqlx::query(
            "SELECT t.* FROM learn_topics t JOIN learn_subjects s ON s.id = t.subject
             WHERE t.id IN (SELECT topic_id FROM learn_fts WHERE learn_fts MATCH ?)
             ORDER BY s.ord, t.ord, t.id",
        )
        .bind(fts)
        .fetch_all(self.pool())
        .await?;
        rows.iter().map(|r| summary(r, lang)).collect()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::content::tests::{sample_problem, temp_dir};
    use crate::content::{PackMeta, build_with_learn};
    use kb_core::Difficulty;

    pub fn sample_learn() -> LearnPack {
        let subject = |id: &str, ord, en: &str, es: &str| PackSubject {
            id: id.into(),
            ord,
            name_en: en.into(),
            name_es: es.into(),
            blurb_en: format!("{en} blurb"),
            blurb_es: format!("{es} resumen"),
            icon: "sort".into(),
        };
        let topic = |id: &str, ord, en: &str, es: &str, theory_es: &str| PackTopic {
            id: id.into(),
            subject: "sorting".into(),
            ord,
            title_en: en.into(),
            title_es: es.into(),
            tags: vec!["sorting".into(), "in-place".into()],
            complexity: LearnComplexity {
                best: "O(n)".into(),
                average: "O(n²)".into(),
                worst: "O(n²)".into(),
                space: "O(1)".into(),
                stable: true,
                in_place: true,
            },
            problems: vec!["gap-a".into()],
            presets: vec!["random".into(), "reversed".into()],
            input: LearnInput {
                min_len: 2,
                max_len: 16,
                min: 1,
                max: 99,
            },
            default_input: vec![5, 3, 8],
            theory_en: format!("{en}: swap neighbours."),
            theory_es: theory_es.into(),
            samples: vec![
                (Language::TypeScript, "function f() {} // @step a\n".into()),
                (Language::Python, "def f(): pass  # @step a\n".into()),
            ],
            quiz: vec![PackQuizQuestion {
                prompt_en: "Next?".into(),
                prompt_es: "¿Siguiente?".into(),
                options: vec![
                    PackQuizOption {
                        en: "Swap".into(),
                        es: "Intercambiar".into(),
                    },
                    PackQuizOption {
                        en: "Stay".into(),
                        es: "Quedarse".into(),
                    },
                ],
                answer: 1,
                explain_en: "Because.".into(),
                explain_es: "Porque.".into(),
            }],
        };
        LearnPack {
            subjects: vec![
                subject("sorting", 2, "Sorting", "Ordenación"),
                subject("foundations", 1, "Foundations", "Fundamentos"),
            ],
            topics: vec![
                topic(
                    "quick-sort",
                    2,
                    "Quick Sort",
                    "Ordenación rápida",
                    "Partición con pivote.",
                ),
                topic(
                    "bubble-sort",
                    1,
                    "Bubble Sort",
                    "Ordenación de burbuja",
                    "Intercambia vecinos.",
                ),
            ],
            paths: vec![PackPath {
                id: "start-here".into(),
                ord: 1,
                title_en: "Start here".into(),
                title_es: "Empieza aquí".into(),
                blurb_en: "First steps".into(),
                blurb_es: "Primeros pasos".into(),
                topics: vec!["bubble-sort".into(), "quick-sort".into()],
            }],
        }
    }

    async fn learn_db(name: &str, learn: &LearnPack) -> Result<ContentDb> {
        let dir = temp_dir(name);
        let path = dir.join("content.db");
        let meta = PackMeta {
            pack_id: "test".into(),
            pack_version: "1.0.0".into(),
            pack_date: None,
        };
        let problems = [sample_problem("gap-a", Difficulty::Easy, 1, true)];
        build_with_learn(&path, &meta, &problems, learn).await?;
        ContentDb::open(&path).await
    }

    #[tokio::test]
    async fn the_catalogue_round_trips_in_order_with_empty_subjects() {
        let db = learn_db("learn-catalog", &sample_learn()).await.unwrap();
        let en = db.learn_catalog(UiLang::En).await.unwrap();
        assert_eq!(en.len(), 2);
        assert_eq!(en[0].id, "foundations");
        assert!(en[0].topics.is_empty(), "a subject with no topics yet");
        assert_eq!(en[1].name, "Sorting");
        let ids: Vec<_> = en[1].topics.iter().map(|t| t.id.as_str()).collect();
        assert_eq!(ids, ["bubble-sort", "quick-sort"]);
        let es = db.learn_catalog(UiLang::Es).await.unwrap();
        assert_eq!(es[1].name, "Ordenación");
        assert_eq!(es[1].topics[0].title, "Ordenación de burbuja");
        assert_eq!(es[1].blurb, "Ordenación resumen");
    }

    #[tokio::test]
    async fn a_topic_round_trips_in_both_languages() {
        let db = learn_db("learn-topic", &sample_learn()).await.unwrap();
        let en = db.learn_topic("bubble-sort", UiLang::En).await.unwrap();
        assert_eq!(en.title, "Bubble Sort");
        assert_eq!(en.subject_name, "Sorting");
        assert_eq!(en.theory, "Bubble Sort: swap neighbours.");
        assert_eq!(en.complexity.worst, "O(n²)");
        assert_eq!(en.input.max_len, 16);
        assert_eq!(en.default_input, vec![5, 3, 8]);
        assert_eq!(en.problems, vec!["gap-a"]);
        assert_eq!(en.presets, vec!["random", "reversed"]);
        let langs: Vec<_> = en.samples.iter().map(|s| s.language).collect();
        assert_eq!(langs, [Language::Python, Language::TypeScript]);
        assert!(en.samples[0].source.contains("# @step a"));
        assert_eq!(en.quiz[0].options, ["Swap", "Stay"]);
        assert_eq!(en.quiz[0].answer, 1);
        let es = db.learn_topic("bubble-sort", UiLang::Es).await.unwrap();
        assert_eq!(es.theory, "Intercambia vecinos.");
        assert_eq!(es.quiz[0].prompt, "¿Siguiente?");
        assert_eq!(es.quiz[0].options[0], "Intercambiar");
        assert_eq!(es.subject_name, "Ordenación");
    }

    #[tokio::test]
    async fn learning_paths_round_trip_in_both_languages() {
        let db = learn_db("learn-paths", &sample_learn()).await.unwrap();
        let en = db.learn_paths(UiLang::En).await.unwrap();
        assert_eq!(en.len(), 1);
        assert_eq!(en[0].id, "start-here");
        assert_eq!(en[0].title, "Start here");
        assert_eq!(en[0].topics, ["bubble-sort", "quick-sort"]);
        let es = db.learn_paths(UiLang::Es).await.unwrap();
        assert_eq!(es[0].title, "Empieza aquí");
        assert_eq!(es[0].blurb, "Primeros pasos");
        // Topic summaries carry their linked problems ("See it animated").
        let catalog = db.learn_catalog(UiLang::En).await.unwrap();
        assert_eq!(catalog[1].topics[0].problems, ["gap-a"]);
    }

    #[tokio::test]
    async fn a_missing_topic_is_not_found() {
        let db = learn_db("learn-missing", &sample_learn()).await.unwrap();
        let err = db.learn_topic("nope", UiLang::En).await.unwrap_err();
        assert!(matches!(err, StoreError::NotFound(_)), "{err:?}");
    }

    #[tokio::test]
    async fn search_matches_either_language_ignoring_accents() {
        let db = learn_db("learn-search", &sample_learn()).await.unwrap();
        let ids = |v: Vec<LearnTopicSummary>| v.into_iter().map(|t| t.id).collect::<Vec<_>>();
        assert_eq!(ids(db.learn_search_en("quick").await), ["quick-sort"]);
        // Spanish words, typed without accents, titles back in English.
        assert_eq!(ids(db.learn_search_en("rapida").await), ["quick-sort"]);
        assert_eq!(ids(db.learn_search_en("particion").await), ["quick-sort"]);
        assert_eq!(
            ids(db.learn_search_en("sorting").await),
            ["bubble-sort", "quick-sort"]
        );
        assert!(db.learn_search_en("heap").await.is_empty());
        assert!(db.learn_search_en("  ** ").await.is_empty());
        let es = db.search_learn("burbuja", UiLang::Es).await.unwrap();
        assert_eq!(es[0].title, "Ordenación de burbuja");
    }

    impl ContentDb {
        async fn learn_search_en(&self, q: &str) -> Vec<LearnTopicSummary> {
            self.search_learn(q, UiLang::En).await.unwrap()
        }
    }

    #[tokio::test]
    async fn a_bad_learn_pack_is_refused() {
        let mut dup = sample_learn();
        dup.topics[1].id = "quick-sort".into();
        assert!(matches!(
            learn_db("learn-dup", &dup).await,
            Err(StoreError::InvalidPack(_))
        ));
        let mut orphan = sample_learn();
        orphan.topics[0].subject = "graphs".into();
        assert!(learn_db("learn-orphan", &orphan).await.is_err());
        let mut place = sample_learn();
        place.topics[0].ord = 1;
        assert!(learn_db("learn-place", &place).await.is_err());
        let mut path = sample_learn();
        path.paths[0].topics.push("heap-sort".into());
        assert!(learn_db("learn-path-topic", &path).await.is_err());
        let mut twice = sample_learn();
        twice.paths.push(twice.paths[0].clone());
        assert!(learn_db("learn-path-twice", &twice).await.is_err());
    }

    #[tokio::test]
    async fn a_pack_without_learn_content_has_an_empty_catalogue() {
        let db = learn_db("learn-empty", &LearnPack::default())
            .await
            .unwrap();
        assert!(db.learn_catalog(UiLang::En).await.unwrap().is_empty());
        assert!(
            db.search_learn("sort", UiLang::En)
                .await
                .unwrap()
                .is_empty()
        );
    }
}
