//! Contributor tooling (features-content M10): `kb-pack new` and `new-topic` scaffold a
//! problem package or a Learn topic with every required file, each holding
//! [`TODO_MARKER`] where something must be written; `kb-pack validate` refuses a file
//! that still holds the marker. `kb-pack preview` renders a problem's statements to one
//! HTML page with a small built-in Markdown renderer (no new dependency).

use std::path::{Path, PathBuf};

use kb_core::Language;
use kb_judge::Signature;

/// What the scaffold leaves for the author; validation refuses files that still hold it.
pub const TODO_MARKER: &str = "TODO(kb-pack)";

/// Files of a package or topic that still hold [`TODO_MARKER`] (paths relative to `dir`).
/// Test and profile data are not scanned (they are generated, and large).
pub fn files_with_todo(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in entries.flatten() {
            let p = e.path();
            let rel = p
                .strip_prefix(dir)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace('\\', "/");
            if p.is_dir() {
                if !(rel == "tests/hidden" || rel.starts_with("profile")) {
                    stack.push(p);
                }
            } else if std::fs::read_to_string(&p).is_ok_and(|s| s.contains(TODO_MARKER)) {
                out.push(rel);
            }
        }
    }
    out.sort();
    out
}

/// How the new problem is solved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewMode {
    Function,
    Program,
}

fn kebab_id(id: &str) -> Result<(), String> {
    let ok = !id.is_empty()
        && id.len() <= 40
        && id.starts_with(|c: char| c.is_ascii_lowercase())
        && !id.ends_with('-')
        && !id.contains("--")
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if ok {
        Ok(())
    } else {
        Err(format!(
            "{id:?}: ids are kebab-case (a-z, 0-9, -), starting with a letter"
        ))
    }
}

fn write_all(dir: &Path, files: &[(String, String)]) -> Result<Vec<PathBuf>, String> {
    if dir.exists() {
        return Err(format!("{} already exists", dir.display()));
    }
    let mut written = Vec::new();
    for (rel, text) in files {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        written.push(path);
    }
    Ok(written)
}

const STATEMENT_FUNCTION_EN: &str = "TODO(kb-pack): the story and the task in two or three short paragraphs. Name the parameters as in `[signature]` (`nums`), and say what to return.

### Example 1

```
nums = [1, 2, 3]
Output: 6
```

TODO(kb-pack): one sentence on why. Examples must equal the public tests (`tests/public/N.json`).

### Constraints

- TODO(kb-pack): the bounds validate.py checks, e.g. `1 ≤ nums.length ≤ 10⁵`
";

const STATEMENT_FUNCTION_ES: &str = "TODO(kb-pack): la historia y la tarea en dos o tres párrafos cortos, con los mismos nombres que `[signature]` (`nums`).

### Ejemplo 1

```
nums = [1, 2, 3]
Output: 6
```

TODO(kb-pack): una frase que lo explique.

### Restricciones

- TODO(kb-pack): los límites que comprueba validate.py
";

const STATEMENT_PROGRAM_EN: &str = "TODO(kb-pack): the story and the task in two or three short paragraphs.

This is a **program problem**: read TODO(kb-pack) from standard input and print TODO(kb-pack) to standard output.

## Input format

- TODO(kb-pack): every line of the input.

## Output format

TODO(kb-pack): every line of the output.

### Example 1

Input:

```
3
1 2 3
```

Output:

```
6
```

### Constraints

- TODO(kb-pack): the bounds validate.py checks
";

const STATEMENT_PROGRAM_ES: &str = "TODO(kb-pack): la historia y la tarea en dos o tres párrafos cortos.

Este es un **problema de programa**: lee TODO(kb-pack) de la entrada estándar y escribe TODO(kb-pack) en la salida estándar.

## Formato de entrada

- TODO(kb-pack): cada línea de la entrada.

## Formato de salida

TODO(kb-pack): cada línea de la salida.

### Ejemplo 1

Entrada:

```
3
1 2 3
```

Salida:

```
6
```

### Restricciones

- TODO(kb-pack): los límites que comprueba validate.py
";

const HINTS_EN: &str = "TODO(kb-pack): a nudge (what to notice).
---
TODO(kb-pack): the key idea.
---
TODO(kb-pack): nearly the full method.
";
const HINTS_ES: &str = "TODO(kb-pack): una pista (en qué fijarse).
---
TODO(kb-pack): la idea clave.
---
TODO(kb-pack): casi el método completo.
";
const EDITORIAL_EN: &str = "## Approach: TODO(kb-pack)

TODO(kb-pack): the method, why it is correct, the complexity, and why the obvious way is too slow at the maximum sizes.
";
const EDITORIAL_ES: &str = "## Enfoque: TODO(kb-pack)

TODO(kb-pack): el método, por qué es correcto, la complejidad y por qué lo obvio es demasiado lento con los tamaños máximos.
";

/// Scaffold `content/problems/<id>` (the folder name is the id). The references come
/// from the judge's own starter code for the placeholder signature, so they already
/// match it; everything to write is marked [`TODO_MARKER`].
pub fn new_problem(problems: &Path, id: &str, mode: NewMode) -> Result<Vec<PathBuf>, String> {
    kebab_id(id)?;
    let dir = problems.join(id);
    let program = mode == NewMode::Program;
    let toml = if program {
        format!(
            "id = \"{id}\"
title = \"TODO(kb-pack): Title\"
title_es = \"TODO(kb-pack): Título\"
difficulty = \"easy\" # easy | medium | hard
tags = [\"input-parsing\"] # TODO(kb-pack): names from tag.names in en.json and es.json
pattern = \"fundamentals\" # TODO(kb-pack): the pattern (kb-pack's PATTERNS)
rank = 5 # 1-10: the order within the pattern
mode = \"program\"

[judge]
checker = \"tokens\" # tokens | tokens-float:<eps> | lines
time_limit_ms = 300 # base (Rust), per test for a program problem (ADR 0017, 0028)
memory_mb = 256
"
        )
    } else {
        format!(
            "id = \"{id}\"
title = \"TODO(kb-pack): Title\"
title_es = \"TODO(kb-pack): Título\"
difficulty = \"easy\" # easy | medium | hard
tags = [\"array\"] # TODO(kb-pack): names from tag.names in en.json and es.json
pattern = \"fundamentals\" # TODO(kb-pack): the pattern (kb-pack's PATTERNS)
rank = 5 # 1-10: the order within the pattern

[signature]
name = \"solve\" # TODO(kb-pack): the function name (camelCase)
params = [{{ name = \"nums\", type = \"int[]\" }}]
returns = \"int\"

[judge]
checker = \"exact\" # exact | unordered | float:<eps>
time_limit_ms = 300 # base (Rust); other languages scale it (ADR 0017)
memory_mb = 256
"
        )
    };
    let sig = Signature::from_json(&serde_json::json!({
        "name": "solve",
        "params": [{"name": "nums", "type": "int[]"}],
        "returns": "int",
    }))
    .map_err(|e| e.to_string())?;
    let mut files: Vec<(String, String)> = vec![
        ("problem.toml".into(), toml),
        (
            "statement.md".into(),
            (if program {
                STATEMENT_PROGRAM_EN
            } else {
                STATEMENT_FUNCTION_EN
            })
            .into(),
        ),
        (
            "statement.es.md".into(),
            (if program {
                STATEMENT_PROGRAM_ES
            } else {
                STATEMENT_FUNCTION_ES
            })
            .into(),
        ),
        ("hints.md".into(), HINTS_EN.into()),
        ("hints.es.md".into(), HINTS_ES.into()),
        ("editorial.md".into(), EDITORIAL_EN.into()),
        ("editorial.es.md".into(), EDITORIAL_ES.into()),
    ];
    let public = if program {
        "{\"stdin\": \"3\\n1 2 3\\n\", \"stdout\": \"6\\n\"}\n"
    } else {
        "{\"input\": {\"nums\": [1, 2, 3]}, \"output\": 6}\n"
    };
    files.push(("tests/public/1.json".into(), public.into()));
    files.push((
        "gen.py".into(),
        format!(
            "\"\"\"TODO(kb-pack): what the inputs look like ({}).\"\"\"
import random


def generate(seed, size):
    rng = random.Random(seed)
    # TODO(kb-pack): \"small\" inputs for the stress test, \"max\" for the limits, and
    # edge shapes (for example when seed % 5 == 0).
    nums = [rng.randint(1, 9) for _ in range(rng.randint(1, 8))]
    return {}


if __name__ == \"__main__\":
    import sys

    args = sys.argv[1:]
    print(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else \"small\"))
",
            if program {
                "stdin text, ADR 0028"
            } else {
                "a dict of params"
            },
            if program {
                "f\"{len(nums)}\\n{' '.join(map(str, nums))}\\n\""
            } else {
                "{\"nums\": nums}"
            },
        ),
    ));
    files.push((
        "validate.py".into(),
        format!(
            "def validate({}):
    # TODO(kb-pack): return the broken constraint as text, or None when the input is valid.
    return None
",
            if program { "text" } else { "inp" }
        ),
    ));
    files.push((
        "solutions/brute.py".into(),
        "# TODO(kb-pack): an obviously correct (slow) solution, written differently from ref.py.\n"
            .into(),
    ));
    for lang in [
        Language::Python,
        Language::TypeScript,
        Language::Php,
        Language::Rust,
    ] {
        let comment = match lang {
            Language::Python => "#",
            _ => "//",
        };
        let body = if program {
            kb_judge::program_starter(lang)
        } else {
            kb_judge::starter_code(lang, &sig)
        }
        .map_err(|e| e.to_string())?;
        let body = if lang == Language::Php {
            body.replacen(
                "<?php\n",
                &format!("<?php\n{comment} {TODO_MARKER}: the reference solution.\n"),
                1,
            )
        } else {
            format!("{comment} {TODO_MARKER}: the reference solution.\n{body}")
        };
        files.push((format!("solutions/ref.{}", lang.extension()), body));
    }
    write_all(&dir, &files)
}

/// Scaffold `content/learn/<subject>/<id>`. The tracer (TypeScript, in the app) and its
/// narration keys come next, then `pnpm learn:cases`.
pub fn new_topic(learn: &Path, subject: &str, id: &str) -> Result<Vec<PathBuf>, String> {
    kebab_id(id)?;
    if !learn.join(subject).is_dir() {
        return Err(format!(
            "subject {subject:?} has no folder in {} (add it to subjects.toml first)",
            learn.display()
        ));
    }
    let dir = learn.join(subject).join(id);
    let snake = id.replace('-', "_");
    let camel: String = id
        .split('-')
        .enumerate()
        .map(|(k, w)| {
            if k == 0 {
                w.to_string()
            } else {
                let mut c = w.chars();
                c.next()
                    .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
                    .unwrap_or_default()
            }
        })
        .collect();
    let files: Vec<(String, String)> = vec![
        (
            "topic.toml".into(),
            format!(
                "id = \"{id}\"
subject = \"{subject}\"
order = 99 # TODO(kb-pack): the place in the subject
title = \"TODO(kb-pack): Title\"
title_es = \"TODO(kb-pack): Título\"
tags = [\"array\"] # TODO(kb-pack): names from learn.tags in the locales
problems = [] # TODO(kb-pack): linked catalogue problems
presets = [\"random\"] # the tracer's preset ids
default = [3, 1, 2]

[complexity]
best = \"TODO(kb-pack)\"
average = \"TODO(kb-pack)\"
worst = \"TODO(kb-pack)\"
space = \"TODO(kb-pack)\"
stable = true
in_place = true

[input]
min_len = 1
max_len = 12
min = 0
max = 99

# The call the sample check makes on the array `a` (ADR 0029).
[sample]
python = \"a = {snake}(a)\"
typescript = \"a = {camel}(a)\"
"
            ),
        ),
        (
            "theory.md".into(),
            "## The idea\n\nTODO(kb-pack)\n\n## Steps\n\nTODO(kb-pack)\n\n## Cost\n\nTODO(kb-pack)\n".into(),
        ),
        (
            "theory.es.md".into(),
            "## La idea\n\nTODO(kb-pack)\n\n## Pasos\n\nTODO(kb-pack)\n\n## Coste\n\nTODO(kb-pack)\n".into(),
        ),
        (
            "sample.py".into(),
            format!(
                "def {snake}(a):\n    # {TODO_MARKER}: the algorithm, with a trailing `# @step <id>` on each step the tracer shows.\n    out = list(a)  # @step copy\n    return out\n"
            ),
        ),
        (
            "sample.ts".into(),
            format!(
                "function {camel}(a: number[]): number[] {{\n  // {TODO_MARKER}: the same algorithm and the same `// @step` ids as sample.py.\n  const out = [...a]; // @step copy\n  return out;\n}}\n"
            ),
        ),
        (
            "quiz.json".into(),
            "[\n  {\n    \"prompt_en\": \"TODO(kb-pack)\",\n    \"prompt_es\": \"TODO(kb-pack)\",\n    \"options\": [\n      { \"en\": \"TODO(kb-pack)\", \"es\": \"TODO(kb-pack)\" },\n      { \"en\": \"TODO(kb-pack)\", \"es\": \"TODO(kb-pack)\" }\n    ],\n    \"answer\": 0,\n    \"explain_en\": \"TODO(kb-pack)\",\n    \"explain_es\": \"TODO(kb-pack)\"\n  },\n  {\n    \"prompt_en\": \"TODO(kb-pack)\",\n    \"prompt_es\": \"TODO(kb-pack)\",\n    \"options\": [\n      { \"en\": \"TODO(kb-pack)\", \"es\": \"TODO(kb-pack)\" },\n      { \"en\": \"TODO(kb-pack)\", \"es\": \"TODO(kb-pack)\" }\n    ],\n    \"answer\": 1,\n    \"explain_en\": \"TODO(kb-pack)\",\n    \"explain_es\": \"TODO(kb-pack)\"\n  }\n]\n".into(),
        ),
    ];
    write_all(&dir, &files)
}

// ---- Preview: a small Markdown renderer --------------------------------------------------

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Wrap each complete pair of `mark`s in `open` / `close`; an unpaired mark stays text.
fn toggle(s: &str, mark: &str, open: &str, close: &str) -> String {
    let parts: Vec<&str> = s.split(mark).collect();
    let marks = parts.len() - 1;
    let paired = marks - marks % 2;
    let mut out = String::new();
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            let k = i - 1;
            out.push_str(if k >= paired {
                mark
            } else if k % 2 == 0 {
                open
            } else {
                close
            });
        }
        out.push_str(p);
    }
    out
}

/// Inline Markdown: `code`, **bold**, *italic* (escaped first; code spans stay literal).
fn inline(text: &str) -> String {
    text.split('`')
        .enumerate()
        .map(|(k, part)| {
            if k % 2 == 1 {
                format!("<code>{}</code>", escape(part))
            } else {
                toggle(
                    &toggle(&escape(part), "**", "<strong>", "</strong>"),
                    "*",
                    "<em>",
                    "</em>",
                )
            }
        })
        .collect()
}

/// Block Markdown: `#` headings, paragraphs, `-` lists, fenced code, `---` rules.
pub fn markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    let mut para: Vec<String> = Vec::new();
    let mut list = false;
    let mut fence: Option<Vec<String>> = None;
    let flush = |html: &mut String, para: &mut Vec<String>, list: &mut bool| {
        if !para.is_empty() {
            html.push_str(&format!("<p>{}</p>\n", inline(&para.join(" "))));
            para.clear();
        }
        if *list {
            html.push_str("</ul>\n");
            *list = false;
        }
    };
    for line in md.lines() {
        if let Some(code) = fence.as_mut() {
            if line.trim_start().starts_with("```") {
                html.push_str(&format!(
                    "<pre><code>{}</code></pre>\n",
                    escape(&code.join("\n"))
                ));
                fence = None;
            } else {
                code.push(line.to_string());
            }
            continue;
        }
        let t = line.trim();
        if t.starts_with("```") {
            flush(&mut html, &mut para, &mut list);
            fence = Some(Vec::new());
        } else if t.is_empty() {
            flush(&mut html, &mut para, &mut list);
        } else if t == "---" {
            flush(&mut html, &mut para, &mut list);
            html.push_str("<hr>\n");
        } else if let Some(level) = (1..=6).find(|&n| t.starts_with(&format!("{} ", "#".repeat(n))))
        {
            flush(&mut html, &mut para, &mut list);
            html.push_str(&format!(
                "<h{level}>{}</h{level}>\n",
                inline(t[level + 1..].trim())
            ));
        } else if let Some(item) = t.strip_prefix("- ") {
            if !para.is_empty() {
                html.push_str(&format!("<p>{}</p>\n", inline(&para.join(" "))));
                para.clear();
            }
            if !list {
                html.push_str("<ul>\n");
                list = true;
            }
            html.push_str(&format!("<li>{}</li>\n", inline(item)));
        } else if list && line.starts_with("  ") {
            // A continued list item: append to the last one.
            if let Some(pos) = html.rfind("</li>") {
                html.insert_str(pos, &format!(" {}", inline(t)));
            }
        } else {
            if list {
                html.push_str("</ul>\n");
                list = false;
            }
            para.push(t.to_string());
        }
    }
    if let Some(code) = fence {
        html.push_str(&format!(
            "<pre><code>{}</code></pre>\n",
            escape(&code.join("\n"))
        ));
    }
    flush(&mut html, &mut para, &mut list);
    html
}

/// Render a problem's statements (EN and ES), hints and editorial into one HTML page at
/// `out_dir/<id>.html`; returns its path.
pub fn preview(problem_dir: &Path, out_dir: &Path) -> Result<PathBuf, String> {
    let id = problem_dir
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("the problem folder has no name")?
        .to_string();
    let read = |name: &str| std::fs::read_to_string(problem_dir.join(name)).unwrap_or_default();
    let toml = read("problem.toml");
    let title_of = |key: &str| {
        toml.lines()
            .find_map(|l| {
                l.strip_prefix(&format!("{key} = \""))
                    .and_then(|r| r.strip_suffix('"'))
            })
            .unwrap_or(&id)
            .to_string()
    };
    let section = |lang: &str,
                   title: String,
                   statement: String,
                   hints: String,
                   editorial: String| {
        let hints_html: String = hints
            .split("\n---")
            .enumerate()
            .filter(|(_, h)| !h.trim().is_empty())
            .map(|(k, h)| {
                format!(
                    "<details><summary>{lang} hint {}</summary>{}</details>\n",
                    k + 1,
                    markdown_to_html(h)
                )
            })
            .collect();
        format!(
            "<section lang=\"{lang}\"><h1>{}</h1>\n{}\n{hints_html}<details><summary>Editorial</summary>{}</details></section>\n",
            escape(&title),
            markdown_to_html(&statement),
            markdown_to_html(&editorial)
        )
    };
    let body = format!(
        "{}{}",
        section(
            "en",
            title_of("title"),
            read("statement.md"),
            read("hints.md"),
            read("editorial.md")
        ),
        section(
            "es",
            title_of("title_es"),
            read("statement.es.md"),
            read("hints.es.md"),
            read("editorial.es.md")
        ),
    );
    let page = format!(
        "<!doctype html>\n<html><head><meta charset=\"utf-8\"><title>{id} (preview)</title>\n<style>\
body{{font-family:system-ui,sans-serif;max-width:60rem;margin:2rem auto;padding:0 1rem;line-height:1.5}}\
section{{border-top:2px solid #ccc;margin-top:2rem}}code{{background:#f2f2f2;padding:0 .2em;border-radius:3px}}\
pre{{background:#f6f6f6;padding:.75rem;overflow:auto}}details{{margin:.5rem 0}}\
</style></head><body>\n<p><em>kb-pack preview of {id}: statements, hints and editorial as Markdown renders them.</em></p>\n{body}</body></html>\n",
        id = escape(&id)
    );
    std::fs::create_dir_all(out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;
    let path = out_dir.join(format!("{id}.html"));
    std::fs::write(&path, page).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("kbp-scaffold-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn a_scaffolded_problem_is_refused_until_every_todo_is_gone() {
        for mode in [NewMode::Function, NewMode::Program] {
            let root = temp(&format!("{mode:?}"));
            let written = new_problem(&root, "new-thing", mode).unwrap();
            assert!(written.len() >= 14, "{written:?}");
            let todo = files_with_todo(&root.join("new-thing"));
            for f in [
                "problem.toml",
                "statement.md",
                "statement.es.md",
                "hints.md",
                "editorial.md",
                "gen.py",
                "validate.py",
                "solutions/brute.py",
                "solutions/ref.py",
                "solutions/ref.rs",
            ] {
                assert!(todo.contains(&f.to_string()), "{mode:?} {f}: {todo:?}");
            }
            // The structure check reports them.
            let (_, errs) = crate::validate::validate_problem_meta(&root.join("new-thing"));
            assert!(errs.iter().any(|e| e.contains(TODO_MARKER)), "{errs:?}");
            // An existing folder is never overwritten.
            assert!(
                new_problem(&root, "new-thing", mode)
                    .unwrap_err()
                    .contains("already exists")
            );
            let _ = std::fs::remove_dir_all(&root);
        }
        assert!(new_problem(&temp("bad"), "Bad_Id", NewMode::Function).is_err());
    }

    #[test]
    fn a_finished_package_has_no_todo() {
        let budget =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/budget-pair");
        assert!(files_with_todo(&budget).is_empty());
        let (_, errs) = crate::validate::validate_problem_meta(&budget);
        assert!(errs.is_empty(), "{errs:?}");
    }

    #[test]
    fn a_scaffolded_topic_is_refused_until_filled() {
        let learn = temp("learn");
        std::fs::create_dir_all(learn.join("sorting")).unwrap();
        new_topic(&learn, "sorting", "gnome-sort").unwrap();
        let todo = files_with_todo(&learn.join("sorting/gnome-sort"));
        assert!(
            todo.contains(&"topic.toml".to_string()) && todo.contains(&"sample.py".to_string()),
            "{todo:?}"
        );
        let ts = std::fs::read_to_string(learn.join("sorting/gnome-sort/sample.ts")).unwrap();
        assert!(ts.contains("function gnomeSort(") && ts.contains("// @step copy"));
        assert!(
            new_topic(&learn, "nope", "x")
                .unwrap_err()
                .contains("subjects.toml")
        );
        let _ = std::fs::remove_dir_all(&learn);
    }

    #[test]
    fn markdown_renders_the_statement_subset_safely() {
        let html = markdown_to_html(
            "A **bold** and *italic* `x < y`.\n\n## Input format\n\n- one\n- two `a`\n  continued\n\n```\n1 <2>\n```\n\n---\nend",
        );
        assert!(
            html.contains("<strong>bold</strong>") && html.contains("<em>italic</em>"),
            "{html}"
        );
        assert!(html.contains("<code>x &lt; y</code>"), "{html}");
        assert!(html.contains("<h2>Input format</h2>"));
        assert!(
            html.contains("<li>two <code>a</code> continued</li>"),
            "{html}"
        );
        assert!(
            html.contains("<pre><code>1 &lt;2&gt;</code></pre>"),
            "{html}"
        );
        assert!(html.contains("<hr>") && html.contains("<p>end</p>"));
        assert!(!markdown_to_html("<script>alert(1)</script>").contains("<script>"));
    }

    #[test]
    fn preview_writes_one_page_with_both_languages() {
        let budget =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems/budget-pair");
        let out = temp("preview");
        let page = std::fs::read_to_string(preview(&budget, &out).unwrap()).unwrap();
        assert!(
            page.contains("<h1>Budget Pair</h1>")
                && page.contains("<h1>Pareja dentro del presupuesto</h1>")
        );
        assert!(page.contains("<h3>Example 1</h3>") && page.contains("<h3>Ejemplo 1</h3>"));
        assert!(page.contains("hint 1") && page.contains("Editorial"));
        let _ = std::fs::remove_dir_all(&out);
    }
}
