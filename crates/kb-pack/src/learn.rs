//! Learn topics (ADR 0029): `content/learn/subjects.toml` and one folder per topic in
//! `content/learn/<subject>/<topic>/`. Validation here, the content.db rows through
//! kb-store, and (`validate --run`) every sample run in the sandbox through the
//! program-mode judge (ADR 0028) against the tracer's committed cases.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use kb_core::{Language, TestCase, TestKind, Verdict};
use kb_judge::{JudgeEnv, Problem, ProblemMode};
use kb_store::{
    LearnComplexity, LearnInput, LearnPack, PackPath, PackQuizQuestion, PackSubject, PackTopic,
};
use serde::Deserialize;
use serde_json::{Map, Value};

/// The languages every topic has a sample in.
pub const SAMPLE_LANGUAGES: [Language; 2] = [Language::Python, Language::TypeScript];
/// The longest flat input a topic may take (a 20 × 20 grid with its header fits).
pub const MAX_INPUT_LEN: u32 = 512;
const FILES: [&str; 6] = [
    "topic.toml",
    "theory.md",
    "theory.es.md",
    "sample.py",
    "sample.ts",
    "quiz.json",
];
/// Per-run CPU base for a sample check: tiny inputs, so generous.
const SAMPLE_TIME_MS: u32 = 1000;
const SAMPLE_MEMORY_MB: u32 = 256;

/// `content/learn` next to a `content/problems` folder.
pub fn learn_dir_for(problems: &Path) -> PathBuf {
    problems
        .parent()
        .map(|p| p.join("learn"))
        .unwrap_or_else(|| PathBuf::from("learn"))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SubjectsToml {
    subject: Vec<SubjectToml>,
}

/// `content/learn/paths.toml` (Phase 6): learning paths, in display order.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PathsToml {
    path: Vec<PathToml>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PathToml {
    id: String,
    title: String,
    title_es: String,
    blurb: String,
    blurb_es: String,
    topics: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SubjectToml {
    id: String,
    name: String,
    name_es: String,
    blurb: String,
    blurb_es: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TopicToml {
    id: String,
    subject: String,
    order: u32,
    title: String,
    title_es: String,
    tags: Vec<String>,
    problems: Vec<String>,
    presets: Vec<String>,
    default: Vec<i64>,
    complexity: LearnComplexity,
    input: LearnInput,
    sample: SampleCalls,
    /// The samples print the sorted input (checked on every case). Default: true in the
    /// `sorting` subject, false elsewhere; quickselect, a sorting topic, sets false.
    #[serde(default)]
    sorts: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SampleCalls {
    python: String,
    typescript: String,
}

impl SampleCalls {
    fn of(&self, l: Language) -> &str {
        match l {
            Language::Python => &self.python,
            _ => &self.typescript,
        }
    }
}

/// `cases.json`: inputs and the tracer's final state for each (written by
/// `pnpm learn:cases`, checked fresh by Vitest).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cases {
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub name: String,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
}

/// One topic as read from disk.
#[derive(Debug)]
pub struct TopicDir {
    pub dir: PathBuf,
    pub topic: PackTopic,
    calls: SampleCalls,
    pub cases: Vec<Case>,
    /// The samples print the sorted input (topic.toml `sorts`, or the subject's default).
    pub sorts: bool,
}

/// Everything under `content/learn`, read and checked. `Err` lists every problem found.
pub struct Learn {
    pub pack: LearnPack,
    pub topics: Vec<TopicDir>,
}

fn kebab(s: &str) -> bool {
    !s.is_empty()
        && s.starts_with(|c: char| c.is_ascii_lowercase())
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Read and validate `learn` (problem ids are checked against `problems`). A missing
/// folder is an empty Learn module.
pub fn read_learn(learn: &Path, problems: &Path) -> Result<Learn, Vec<String>> {
    let mut errors = Vec::new();
    if !learn.is_dir() {
        return Ok(Learn {
            pack: LearnPack::default(),
            topics: Vec::new(),
        });
    }
    let subjects = match fs::read_to_string(learn.join("subjects.toml"))
        .map_err(|e| e.to_string())
        .and_then(|s| toml::from_str::<SubjectsToml>(&s).map_err(|e| e.to_string()))
    {
        Ok(s) => s.subject,
        Err(e) => return Err(vec![format!("subjects.toml: {e}")]),
    };
    let mut pack = LearnPack::default();
    let mut seen = BTreeSet::new();
    for (k, s) in subjects.into_iter().enumerate() {
        if !kebab(&s.id) || !seen.insert(s.id.clone()) {
            errors.push(format!(
                "subjects.toml: subject id {:?} is not unique kebab-case",
                s.id
            ));
        }
        for (what, v) in [
            ("name", &s.name),
            ("name_es", &s.name_es),
            ("blurb", &s.blurb),
            ("blurb_es", &s.blurb_es),
            ("icon", &s.icon),
        ] {
            if v.trim().is_empty() {
                errors.push(format!("subjects.toml: {}: empty {what}", s.id));
            }
        }
        pack.subjects.push(PackSubject {
            id: s.id,
            ord: k as u32 + 1,
            name_en: s.name,
            name_es: s.name_es,
            blurb_en: s.blurb,
            blurb_es: s.blurb_es,
            icon: s.icon,
        });
    }

    let mut topics = Vec::new();
    let mut entries: Vec<PathBuf> = fs::read_dir(learn)
        .map(|r| {
            r.flatten()
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .collect()
        })
        .unwrap_or_default();
    entries.sort();
    for subject_dir in entries {
        let subject = name_of(&subject_dir);
        if !seen.contains(&subject) {
            errors.push(format!(
                "{}: not a subject in subjects.toml",
                subject_dir.display()
            ));
            continue;
        }
        let mut dirs: Vec<PathBuf> = fs::read_dir(&subject_dir)
            .map(|r| {
                r.flatten()
                    .map(|e| e.path())
                    .filter(|p| p.is_dir())
                    .collect()
            })
            .unwrap_or_default();
        dirs.sort();
        for dir in dirs {
            match read_topic(&dir, &subject, problems) {
                Ok(t) => topics.push(t),
                Err(es) => errors.extend(es.into_iter().map(|e| format!("{}: {e}", dir.display()))),
            }
        }
    }

    // Order unique within a subject; links to other topics resolve.
    let mut places = BTreeMap::new();
    for t in &topics {
        if let Some(other) =
            places.insert((t.topic.subject.clone(), t.topic.ord), t.topic.id.clone())
        {
            errors.push(format!(
                "{}: order {} is also used by {other}",
                t.topic.id, t.topic.ord
            ));
        }
    }
    let known: BTreeSet<String> = topics
        .iter()
        .map(|t| format!("{}/{}", t.topic.subject, t.topic.id))
        .collect();
    for t in &topics {
        for (file, text) in [
            ("theory.md", &t.topic.theory_en),
            ("theory.es.md", &t.topic.theory_es),
        ] {
            for link in learn_links(text) {
                if !known.contains(&link) {
                    errors.push(format!(
                        "{}: {file} links to unknown topic learn:{link}",
                        t.topic.id
                    ));
                }
            }
        }
    }
    // Learning paths (optional file): known topics, each once per path.
    let path_file = learn.join("paths.toml");
    if path_file.is_file() {
        match fs::read_to_string(&path_file)
            .map_err(|e| e.to_string())
            .and_then(|s| toml::from_str::<PathsToml>(&s).map_err(|e| e.to_string()))
        {
            Err(e) => errors.push(format!("paths.toml: {e}")),
            Ok(file) => {
                let ids: BTreeSet<&str> = topics.iter().map(|t| t.topic.id.as_str()).collect();
                let mut seen_paths = BTreeSet::new();
                for (k, p) in file.path.into_iter().enumerate() {
                    if !kebab(&p.id) || !seen_paths.insert(p.id.clone()) {
                        errors.push(format!(
                            "paths.toml: path id {:?} is not unique kebab-case",
                            p.id
                        ));
                    }
                    for (what, v) in [
                        ("title", &p.title),
                        ("title_es", &p.title_es),
                        ("blurb", &p.blurb),
                        ("blurb_es", &p.blurb_es),
                    ] {
                        if v.trim().is_empty() {
                            errors.push(format!("paths.toml: {}: empty {what}", p.id));
                        }
                    }
                    if p.topics.is_empty() {
                        errors.push(format!("paths.toml: {}: no topics", p.id));
                    }
                    let mut in_path = BTreeSet::new();
                    for t in &p.topics {
                        if !ids.contains(t.as_str()) {
                            errors.push(format!("paths.toml: {}: unknown topic {t}", p.id));
                        }
                        if !in_path.insert(t.as_str()) {
                            errors.push(format!("paths.toml: {}: {t} listed twice", p.id));
                        }
                    }
                    pack.paths.push(PackPath {
                        id: p.id,
                        ord: k as u32 + 1,
                        title_en: p.title,
                        title_es: p.title_es,
                        blurb_en: p.blurb,
                        blurb_es: p.blurb_es,
                        topics: p.topics,
                    });
                }
            }
        }
    }

    // Tracers live in the app (ADR 0029); check each topic has one when the app is here.
    if let Some(tracers) = tracers_dir(learn) {
        for t in &topics {
            if !tracers.join(format!("{}.ts", t.topic.id)).is_file() {
                errors.push(format!(
                    "{}: no tracer at {}",
                    t.topic.id,
                    tracers.join(format!("{}.ts", t.topic.id)).display()
                ));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    pack.topics = topics.iter().map(|t| t.topic.clone()).collect();
    Ok(Learn { pack, topics })
}

/// `apps/desktop/src/learn/tracers` in the repository holding `learn`, if present.
fn tracers_dir(learn: &Path) -> Option<PathBuf> {
    let dir = learn
        .parent()?
        .parent()?
        .join("apps/desktop/src/learn/tracers");
    dir.is_dir().then_some(dir)
}

fn name_of(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// `learn:<subject>/<topic>` link targets in a Markdown text.
fn learn_links(text: &str) -> Vec<String> {
    text.match_indices("(learn:")
        .filter_map(|(at, m)| {
            let rest = &text[at + m.len()..];
            rest.find(')').map(|end| rest[..end].to_string())
        })
        .collect()
}

/// `##` headings in order.
fn headings(text: &str) -> Vec<&str> {
    text.lines().filter(|l| l.starts_with("## ")).collect()
}

/// `@step` ids in `source` for a language's comment marker; errors for malformed or
/// repeated markers.
pub fn markers(source: &str, comment: &str) -> Result<Vec<String>, Vec<String>> {
    let mut ids = Vec::new();
    let mut errors = Vec::new();
    for (k, line) in source.lines().enumerate() {
        let Some(at) = line.find("@step") else {
            continue;
        };
        let before = line[..at].trim_end();
        let id = line[at + "@step".len()..].trim();
        let ok = before.ends_with(comment)
            && !before[..before.len() - comment.len()].trim().is_empty()
            && kebab(id);
        if !ok {
            errors.push(format!(
                "line {}: a marker is a trailing `{comment} @step <id>` comment on a code line",
                k + 1
            ));
        } else if ids.iter().any(|x| x == id) {
            errors.push(format!("line {}: marker {id:?} is used twice", k + 1));
        } else {
            ids.push(id.to_string());
        }
    }
    if errors.is_empty() {
        Ok(ids)
    } else {
        Err(errors)
    }
}

fn read_topic(dir: &Path, subject: &str, problems: &Path) -> Result<TopicDir, Vec<String>> {
    let mut errors = Vec::new();
    for f in FILES.iter().chain(["cases.json"].iter()) {
        if !dir.join(f).is_file() {
            errors.push(format!("missing {f}"));
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let read = |f: &str| fs::read_to_string(dir.join(f)).map_err(|e| vec![format!("{f}: {e}")]);
    let t: TopicToml =
        toml::from_str(&read("topic.toml")?).map_err(|e| vec![format!("topic.toml: {e}")])?;

    let folder = name_of(dir);
    if t.id != folder {
        errors.push(format!("id {:?} does not match the folder", t.id));
    }
    if t.subject != subject {
        errors.push(format!(
            "subject {:?} does not match the folder {subject:?}",
            t.subject
        ));
    }
    if t.order == 0 {
        errors.push("order starts at 1".into());
    }
    if t.title.trim().is_empty() || t.title_es.trim().is_empty() {
        errors.push("title and title_es are required".into());
    }
    if t.tags.is_empty() || !t.tags.iter().all(|x| kebab(x)) {
        errors.push("tags: at least one, kebab-case".into());
    }
    for p in &t.problems {
        if !problems.join(p).join("problem.toml").is_file() {
            errors.push(format!("linked problem {p:?} is not in the catalogue"));
        }
    }
    // Preset ids are the tracer's (Vitest checks each exists there and has a name in both
    // locales); here they only need to be well-formed.
    if t.presets.is_empty() || t.presets.iter().any(|p| !kebab(p)) {
        errors.push("presets: one or more kebab-case ids".into());
    }
    let lim = t.input;
    let lim_ok = lim.min_len >= 1
        && lim.min_len <= lim.max_len
        && lim.max_len <= MAX_INPUT_LEN
        && lim.min <= lim.max;
    if !lim_ok {
        errors.push(format!(
            "input: need 1 ≤ min_len ≤ max_len ≤ {MAX_INPUT_LEN} and min ≤ max"
        ));
    }
    let fits = |xs: &[i64]| {
        (lim.min_len as usize..=lim.max_len as usize).contains(&xs.len())
            && xs
                .iter()
                .all(|&v| v >= lim.min as i64 && v <= lim.max as i64)
    };
    if !fits(&t.default) {
        errors.push("default input is outside the input limits".into());
    }
    let c = &t.complexity;
    if [&c.best, &c.average, &c.worst, &c.space]
        .iter()
        .any(|s| s.trim().is_empty())
    {
        errors.push("complexity: best, average, worst and space are required".into());
    }

    // Theory: EN and ES with the same sections.
    let theory_en = read("theory.md")?;
    let theory_es = read("theory.es.md")?;
    let (he, hs) = (headings(&theory_en), headings(&theory_es));
    if he.is_empty() || he.len() != hs.len() {
        errors.push(format!(
            "theory.md and theory.es.md need the same number of `##` sections ({} vs {})",
            he.len(),
            hs.len()
        ));
    }

    // Samples: the same markers in both.
    let py = read("sample.py")?;
    let ts = read("sample.ts")?;
    let mut sets = Vec::new();
    for (f, src, comment) in [("sample.py", &py, "#"), ("sample.ts", &ts, "//")] {
        match markers(src, comment) {
            Ok(ids) if ids.is_empty() => errors.push(format!("{f}: no `@step` markers")),
            Ok(ids) => sets.push(ids.into_iter().collect::<BTreeSet<_>>()),
            Err(es) => errors.extend(es.into_iter().map(|e| format!("{f}: {e}"))),
        }
    }
    if sets.len() == 2 && sets[0] != sets[1] {
        errors.push(format!(
            "sample.py and sample.ts mark different steps: {:?} vs {:?}",
            sets[0], sets[1]
        ));
    }
    for l in SAMPLE_LANGUAGES {
        if t.sample.of(l).trim().is_empty() {
            errors.push(format!("[sample] needs the {} call", l.id()));
        }
    }

    // Quiz.
    let quiz: Vec<PackQuizQuestion> = match serde_json::from_str(&read("quiz.json")?) {
        Ok(q) => q,
        Err(e) => {
            errors.push(format!("quiz.json: {e}"));
            Vec::new()
        }
    };
    if !(2..=5).contains(&quiz.len()) {
        errors.push(format!("quiz.json: 2 to 5 questions, found {}", quiz.len()));
    }
    for (k, q) in quiz.iter().enumerate() {
        let texts = [&q.prompt_en, &q.prompt_es, &q.explain_en, &q.explain_es];
        let options_ok = (2..=4).contains(&q.options.len())
            && q.options
                .iter()
                .all(|o| !o.en.trim().is_empty() && !o.es.trim().is_empty());
        if texts.iter().any(|s| s.trim().is_empty()) || !options_ok {
            errors.push(format!(
                "quiz.json question {}: prompt and explanation in both languages, 2 to 4 options",
                k + 1
            ));
        }
        if q.answer as usize >= q.options.len() {
            errors.push(format!(
                "quiz.json question {}: answer {} is not an option",
                k + 1,
                q.answer
            ));
        }
    }

    // Cases.
    let sorts = t.sorts.unwrap_or(subject == "sorting");
    let cases = match serde_json::from_str::<Cases>(&read("cases.json")?) {
        Ok(c) => c.cases,
        Err(e) => {
            errors.push(format!("cases.json: {e}"));
            Vec::new()
        }
    };
    if cases.is_empty() {
        errors.push("cases.json: no cases (run `pnpm learn:cases`)".into());
    }
    for c in &cases {
        if !fits(&c.input) {
            errors.push(format!("cases.json {:?}: input outside the limits", c.name));
        }
        if sorts && !is_sort_of(&c.output, &c.input) {
            errors.push(format!(
                "cases.json {:?}: the tracer's final state is not the sorted input",
                c.name
            ));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(TopicDir {
        dir: dir.to_path_buf(),
        topic: PackTopic {
            id: t.id,
            subject: t.subject,
            ord: t.order,
            title_en: t.title,
            title_es: t.title_es,
            tags: t.tags,
            complexity: t.complexity,
            problems: t.problems,
            presets: t.presets,
            input: t.input,
            default_input: t.default,
            theory_en,
            theory_es,
            samples: vec![(Language::Python, py), (Language::TypeScript, ts)],
            quiz,
        },
        calls: t.sample,
        cases,
        sorts,
    })
}

/// `output` is `input` sorted ascending.
pub fn is_sort_of(output: &[i64], input: &[i64]) -> bool {
    let mut sorted = input.to_vec();
    sorted.sort_unstable();
    output == sorted
}

/// Validate `content/learn` and print one line per topic. Returns the failure count.
pub fn validate_learn(learn: &Path, problems: &Path) -> usize {
    if !learn.is_dir() {
        return 0;
    }
    match read_learn(learn, problems) {
        Ok(l) => {
            for t in &l.topics {
                println!(
                    "ok    learn {}/{} ({} cases, {} quiz)",
                    t.topic.subject,
                    t.topic.id,
                    t.cases.len(),
                    t.topic.quiz.len()
                );
            }
            println!(
                "\n{} subject(s), {} topic(s), 0 failed",
                l.pack.subjects.len(),
                l.topics.len()
            );
            0
        }
        Err(errors) => {
            println!("FAIL  {}", learn.display());
            for e in &errors {
                println!("        - {e}");
            }
            errors.len()
        }
    }
}

/// The program the sample check runs: the sample wrapped in the language's learn driver
/// with the topic's call (`languages/<lang>/learn_driver.*`).
pub fn sample_program(source: &str, language: Language, call: &str) -> String {
    let driver = match language {
        Language::Python => include_str!("../../../languages/python/learn_driver.py"),
        _ => include_str!("../../../languages/typescript/learn_driver.ts"),
    };
    driver
        .replace("{{call}}", call)
        .replace("{{sample}}", source.trim_end())
}

fn tokens(xs: &[i64]) -> String {
    let parts: Vec<String> = xs.iter().map(i64::to_string).collect();
    format!("{}\n", parts.join(" "))
}

/// The synthetic program problem that checks one topic's sample on its cases.
pub fn sample_problem(t: &TopicDir) -> Result<Problem, String> {
    let tests = t
        .cases
        .iter()
        .enumerate()
        .map(|(k, c)| {
            let mut input = Map::new();
            input.insert("stdin".into(), Value::String(tokens(&c.input)));
            TestCase {
                kind: TestKind::Hidden,
                index: k as u32 + 1,
                input,
                expected: Value::String(tokens(&c.output)),
            }
        })
        .collect();
    Problem::from_parts(
        &format!("learn-{}", t.topic.id),
        ProblemMode::Program,
        &Value::Null,
        "tokens",
        SAMPLE_TIME_MS,
        SAMPLE_MEMORY_MB,
        tests,
    )
    .map_err(|e| e.to_string())
}

/// Run one topic's sample in `language` on every case. `Ok` means every output matched
/// the tracer's final state.
pub fn run_sample(t: &TopicDir, language: Language, env: &JudgeEnv) -> Result<(), String> {
    let problem = sample_problem(t)?;
    let source = t
        .topic
        .samples
        .iter()
        .find(|(l, _)| *l == language)
        .map(|(_, s)| s.as_str())
        .ok_or_else(|| format!("no {} sample", language.id()))?;
    let program = sample_program(source, language, t.calls.of(language));
    let report = kb_judge::judge(&problem, &problem.tests, language, &program, env)
        .map_err(|e| e.to_string())?;
    if report.verdict == Verdict::Accepted {
        return Ok(());
    }
    let detail = report
        .first_failure
        .map(|f| {
            format!(
                "{:?}: {}",
                f.message,
                serde_json::to_string(&f.actual).unwrap_or_default()
            )
        })
        .or_else(|| report.compile_output.map(|m| format!("{m:?}")))
        .unwrap_or_default();
    Err(format!("{:?} {detail}", report.verdict))
}

/// Which topics `validate --learn --run` runs in the sandbox: every topic (the default),
/// the listed ids, or one subject's topics (`--topics a,b`, `--subject s`).
#[derive(Debug, Default, Clone)]
pub struct TopicFilter {
    pub topics: Vec<String>,
    pub subject: Option<String>,
}

impl TopicFilter {
    /// The selected topics, in catalogue order. An id or subject that matches nothing is
    /// an error, so a typo never passes as "0 failed".
    pub fn select<'a>(&self, all: &'a [TopicDir]) -> Result<Vec<&'a TopicDir>, String> {
        let unknown: Vec<&str> = self
            .topics
            .iter()
            .filter(|id| !all.iter().any(|t| &t.topic.id == *id))
            .map(String::as_str)
            .collect();
        if !unknown.is_empty() {
            return Err(format!("unknown topic id(s): {}", unknown.join(", ")));
        }
        if let Some(s) = &self.subject
            && !all.iter().any(|t| &t.topic.subject == s)
        {
            return Err(format!("no topics in subject {s:?}"));
        }
        Ok(all
            .iter()
            .filter(|t| self.topics.is_empty() || self.topics.contains(&t.topic.id))
            .filter(|t| self.subject.as_ref().is_none_or(|s| &t.topic.subject == s))
            .collect())
    }
}

/// `validate --run` for topics: every sample in every language, for the topics `filter`
/// selects. Returns the failures.
pub fn run_samples(learn: &Path, problems: &Path, env: &JudgeEnv, filter: &TopicFilter) -> usize {
    let all = match read_learn(learn, problems) {
        Ok(l) => l.topics,
        Err(_) => return 1, // validate already printed why
    };
    let topics = match filter.select(&all) {
        Ok(t) => t,
        Err(e) => {
            println!("FAIL  {e}");
            return 1;
        }
    };
    let mut failed = 0;
    for t in topics {
        for l in SAMPLE_LANGUAGES {
            match run_sample(t, l, env) {
                Ok(()) => println!(
                    "ok    learn {} sample.{} matches the tracer on {} cases",
                    t.topic.id,
                    l.extension(),
                    t.cases.len()
                ),
                Err(e) => {
                    failed += 1;
                    println!("FAIL  learn {} sample.{}: {e}", t.topic.id, l.extension());
                }
            }
        }
    }
    failed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn the_bundled_topics_validate() {
        let l = read_learn(
            &repo().join("content/learn"),
            &repo().join("content/problems"),
        )
        .unwrap_or_else(|e| panic!("{e:#?}"));
        assert_eq!(l.pack.subjects.len(), 13);
        let ids: Vec<_> = l.topics.iter().map(|t| t.topic.id.as_str()).collect();
        // Every topic shipped so far (read in folder order; the app orders by `order`).
        let mut ids: Vec<_> = ids;
        ids.sort_unstable();
        for want in [
            "big-o-growth",
            "bit-manipulation",
            "bubble-sort",
            "dynamic-array",
            "insertion-sort",
            "prefix-sums",
            "quick-sort",
            "recursion-call-stack",
            "towers-of-hanoi",
        ] {
            assert!(ids.binary_search(&want).is_ok(), "{want} is missing");
        }
        for t in &l.topics {
            let lim = t.topic.input;
            if lim.max_len == 1 {
                // A count input (n): one case per n.
                let all = (lim.max - lim.min + 1) as usize;
                assert_eq!(t.cases.len(), all, "{}: every n", t.topic.id);
            } else {
                // At least one preset plus the 20 random inputs.
                assert!(t.cases.len() >= 21, "{}: presets + 20 random", t.topic.id);
            }
        }
    }

    #[test]
    fn every_tag_and_icon_has_a_name_in_the_app() {
        let l = read_learn(
            &repo().join("content/learn"),
            &repo().join("content/problems"),
        )
        .unwrap();
        let ui = repo().join("apps/desktop/src");
        for lang in ["en", "es"] {
            let text = fs::read_to_string(ui.join(format!("i18n/locales/{lang}.json"))).unwrap();
            let json: Value = serde_json::from_str(&text).unwrap();
            for t in &l.topics {
                for tag in &t.topic.tags {
                    assert!(
                        json["learn"]["tags"][tag].is_string(),
                        "{lang}.json lacks learn.tags.{tag}"
                    );
                }
            }
        }
        let icons = fs::read_to_string(ui.join("learn/icons.ts")).unwrap();
        for s in &l.pack.subjects {
            assert!(
                icons.contains(&format!("\"{}\":", s.icon)),
                "icons.ts lacks {:?}",
                s.icon
            );
        }
    }

    #[test]
    fn markers_must_be_trailing_unique_kebab_comments() {
        assert_eq!(
            markers("x = 1  # @step a-b\ny = 2\n", "#").unwrap(),
            ["a-b"]
        );
        assert!(
            markers("# @step lonely\n", "#").is_err(),
            "a comment-only line"
        );
        assert!(markers("x = 1  # @step Bad\n", "#").is_err());
        assert!(markers("x = 1  # @step a\ny = 1  # @step a\n", "#").is_err());
        assert!(
            markers("x = 1;  # @step a\n", "//").is_err(),
            "wrong comment"
        );
        assert_eq!(markers("f(); // @step go\n", "//").unwrap(), ["go"]);
    }

    #[test]
    fn links_and_headings_are_found() {
        let t = "## A\nsee [x](learn:sorting/quick-sort) and [y](learn:graphs/bfs).\n## B\n";
        assert_eq!(learn_links(t), ["sorting/quick-sort", "graphs/bfs"]);
        assert_eq!(headings(t), ["## A", "## B"]);
    }

    /// Copy a bundled topic into a temp learn folder, break it with `edit`, and return
    /// the errors.
    fn broken(name: &str, edit: impl Fn(&Path)) -> Vec<String> {
        let root =
            std::env::temp_dir().join(format!("kb-pack-learn-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let learn = root.join("content/learn");
        let topic = learn.join("sorting/quick-sort");
        fs::create_dir_all(&topic).unwrap();
        fs::copy(
            repo().join("content/learn/subjects.toml"),
            learn.join("subjects.toml"),
        )
        .unwrap();
        let src = repo().join("content/learn/sorting/quick-sort");
        for f in FILES.iter().chain(["cases.json"].iter()) {
            fs::copy(src.join(f), topic.join(f)).unwrap();
        }
        // Theory links to other sorting topics; keep the copy self-contained.
        for f in ["theory.md", "theory.es.md"] {
            let text = fs::read_to_string(topic.join(f)).unwrap();
            let text = text
                .replace("(learn:sorting/insertion-sort)", "")
                .replace("(learn:sorting/bubble-sort)", "")
                .replace("(learn:sorting/quick-sort-3way)", "");
            fs::write(topic.join(f), text).unwrap();
        }
        let ok = read_learn(&learn, &repo().join("content/problems"));
        assert!(ok.is_ok(), "the unbroken copy validates: {:?}", ok.err());
        edit(&topic);
        let errors = read_learn(&learn, &repo().join("content/problems"))
            .err()
            .unwrap_or_default();
        let _ = fs::remove_dir_all(&root);
        errors
    }

    fn rewrite(p: &Path, from: &str, to: &str) {
        let text = fs::read_to_string(p).unwrap();
        assert!(text.contains(from), "{} lacks {from:?}", p.display());
        fs::write(p, text.replacen(from, to, 1)).unwrap();
    }

    fn has(errors: &[String], needle: &str) -> bool {
        errors.iter().any(|e| e.contains(needle))
    }

    #[test]
    fn learning_paths_are_checked() {
        let path = |id: &str, title: &str, topics: &str| {
            format!(
                "[[path]]\nid = \"{id}\"\ntitle = \"{title}\"\ntitle_es = \"T\"\nblurb = \"b\"\n\
                 blurb_es = \"b\"\ntopics = [{topics}]\n"
            )
        };
        let write = |t: &Path, body: String| {
            fs::write(
                t.parent().unwrap().parent().unwrap().join("paths.toml"),
                body,
            )
            .unwrap()
        };
        let e = broken("path-ok", |t| {
            write(t, path("start", "Start", "\"quick-sort\""))
        });
        assert!(e.is_empty(), "{e:?}");
        let e = broken("path-unknown", |t| {
            write(t, path("start", "Start", "\"heap-sort\""))
        });
        assert!(has(&e, "unknown topic heap-sort"), "{e:?}");
        let e = broken("path-twice", |t| {
            write(t, path("start", "Start", "\"quick-sort\", \"quick-sort\""))
        });
        assert!(has(&e, "quick-sort listed twice"), "{e:?}");
        let e = broken("path-id", |t| {
            write(t, path("Start Here", "Start", "\"quick-sort\""))
        });
        assert!(has(&e, "not unique kebab-case"), "{e:?}");
        let e = broken("path-title", |t| {
            write(t, path("start", " ", "\"quick-sort\""))
        });
        assert!(has(&e, "empty title"), "{e:?}");
        let e = broken("path-none", |t| write(t, path("start", "Start", "")));
        assert!(has(&e, "no topics"), "{e:?}");
    }

    #[test]
    fn broken_topics_are_refused_with_a_reason() {
        let e = broken("missing", |t| {
            fs::remove_file(t.join("theory.es.md")).unwrap()
        });
        assert!(has(&e, "missing theory.es.md"), "{e:?}");
        let e = broken("problem", |t| {
            rewrite(&t.join("topic.toml"), "\"score-ranks\"", "\"no-such\"")
        });
        assert!(has(&e, "linked problem \"no-such\""), "{e:?}");
        let e = broken("marker", |t| {
            rewrite(&t.join("sample.ts"), "@step place", "@step put")
        });
        assert!(has(&e, "mark different steps"), "{e:?}");
        let e = broken("twice", |t| {
            rewrite(&t.join("sample.py"), "@step place", "@step swap")
        });
        assert!(has(&e, "used twice"), "{e:?}");
        let e = broken("quiz", |t| {
            rewrite(&t.join("quiz.json"), "\"answer\": 0", "\"answer\": 7")
        });
        assert!(has(&e, "answer 7 is not an option"), "{e:?}");
        let e = broken("headings", |t| {
            rewrite(&t.join("theory.es.md"), "## Pasos", "Pasos")
        });
        assert!(has(&e, "same number of `##` sections"), "{e:?}");
        let e = broken("link", |t| {
            rewrite(
                &t.join("theory.md"),
                "## The idea",
                "## The idea\n[x](learn:sorting/heap-sort)",
            )
        });
        assert!(has(&e, "unknown topic learn:sorting/heap-sort"), "{e:?}");
        let e = broken("cases", |t| {
            rewrite(&t.join("cases.json"), "\"output\": [", "\"output\": [99, ")
        });
        assert!(has(&e, "not the sorted input"), "{e:?}");
        let e = broken("preset", |t| {
            // Existence is the tracer's (checked by Vitest); kb-pack refuses malformed ids.
            rewrite(&t.join("topic.toml"), "\"few-unique\"", "\"Zig Zag\"")
        });
        assert!(has(&e, "presets"), "{e:?}");
        let e = broken("id", |t| {
            rewrite(
                &t.join("topic.toml"),
                "id = \"quick-sort\"",
                "id = \"quick\"",
            )
        });
        assert!(has(&e, "does not match the folder"), "{e:?}");
        let e = broken("default", |t| {
            rewrite(&t.join("topic.toml"), "default = [38", "default = [380")
        });
        assert!(has(&e, "default input"), "{e:?}");
    }

    #[test]
    fn the_topic_filter_selects_and_refuses_unknown_ids() {
        let learn = read_learn(
            &repo().join("content/learn"),
            &repo().join("content/problems"),
        )
        .expect("bundled topics read");
        let ids = |f: &TopicFilter| -> Vec<String> {
            f.select(&learn.topics)
                .unwrap()
                .iter()
                .map(|t| t.topic.id.clone())
                .collect()
        };
        assert_eq!(ids(&TopicFilter::default()).len(), learn.topics.len());
        let two = TopicFilter {
            topics: vec!["quick-sort".into(), "bubble-sort".into()],
            subject: None,
        };
        // Folder order, not the order given.
        assert_eq!(ids(&two), ["bubble-sort", "quick-sort"]);
        let sorting = TopicFilter {
            topics: vec![],
            subject: Some("sorting".into()),
        };
        let in_sorting = ids(&sorting);
        assert!(in_sorting.contains(&"quick-sort".to_string()));
        assert!(
            learn
                .topics
                .iter()
                .filter(|t| t.topic.subject == "sorting")
                .count()
                == in_sorting.len()
        );
        let typo = TopicFilter {
            topics: vec!["bubble-sort".into(), "bubble-srot".into()],
            subject: None,
        };
        assert_eq!(
            typo.select(&learn.topics).unwrap_err(),
            "unknown topic id(s): bubble-srot"
        );
        let nothing = TopicFilter {
            topics: vec![],
            subject: Some("astrology".into()),
        };
        assert!(nothing.select(&learn.topics).is_err());
    }

    #[test]
    fn the_sample_program_wraps_the_sample_in_the_driver() {
        let p = sample_program("def f(a):\n    a.sort()\n", Language::Python, "f(a)");
        assert!(p.starts_with("def f(a):"));
        assert!(p.contains("\nf(a)\n"));
        assert!(!p.contains("{{"));
        let t = sample_program("function f(a: number[]) {}\n", Language::TypeScript, "f(a)");
        assert!(t.contains("function f(a: number[]) {}\n"));
        assert!(t.contains("\nf(a);\n"));
        assert!(!t.contains("{{"));
        // The driver's import stays first (once needed by `node --check`; ADR 0029).
        let first_code = t.lines().find(|l| !l.starts_with("//")).unwrap();
        assert!(first_code.starts_with("import "), "{first_code}");
    }
}
