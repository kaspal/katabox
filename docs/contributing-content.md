# Contributing content: problems and Learn topics

This guide takes you from an idea to a package that passes every check. The rules
themselves are in [problem-authoring.md](problem-authoring.md) (problems) and
[ADR 0029](adr/0029-learn-module.md) (Learn); `content/problems/budget-pair/` is a
complete function problem, `content/problems/ticket-tally/` a complete program problem.

Everything below runs from the repository root. The sandbox checks need the toolchains:
`scripts/fetch-toolchains.ps1` (Windows; every language) or `scripts/fetch-toolchains.sh`
(Linux; Python and TypeScript).

## A problem, step by step

The worked example is a small program problem, `tidy-shelves`: read the books on each
shelf and print how many are out of order.

### 1. Scaffold

```sh
cargo run -p kb-pack -- new tidy-shelves --mode program
```

This writes `content/problems/tidy-shelves/` with every file a package needs:
`problem.toml`, the statements, hints and editorial in English and Spanish, `gen.py`,
`validate.py`, `solutions/brute.py`, `solutions/ref.{py,ts,php,rs}` and one public test.
Every place where you must write something says `TODO(kb-pack)`. Use
`--mode function` (the default) for a problem solved by a function; its references
already match the placeholder signature, so rename it in `problem.toml` and all four
references together.

`--mode sql` writes an SQL package instead (ADR 0031): `schema.sql` (`CREATE TABLE` only),
`solutions/ref.sql`, `solutions/wrong.sql` (a plausible mistake the tests must catch with
Wrong Answer), and `solutions/brute.py`, whose `solve(tables)` computes the expected
`{"columns": [...], "rows": [[...]]}` in plain Python. Tests are
`{"input": {"tables": {...}}, "output": {"columns": [...], "rows": [...]}}`;
`kb-pack generate` fills the outputs from `ref.sql`. The statement names the dialect
(SQLite) and the pattern is one of `sql-basics`, `sql-joins`, `sql-aggregation` and
`sql-advanced`. `validate --run` checks the validator, `ref.sql` (Accepted, within half
its limit), `wrong.sql` (Wrong Answer) and 500 generated inputs against `brute.py`.

### 2. Fill it in

- **`problem.toml`:** title (EN and ES), difficulty, tags (each needs a name in
  `tag.names` of both `en.json` and `es.json`), pattern, rank, and the checker.
- **Statements:** original wording only. For a program problem, the `## Input format`
  and `## Output format` sections, then one `### Example N` per public test, each with
  the test's stdin and stdout exactly.
- **`tests/public/*.json`:** 3 to 5 small, hand-checked examples.
- **`validate.py`:** return the broken constraint as text, or `None`. It is the source
  of truth for the constraints you list in the statements.
- **`gen.py`:** `generate(seed, size)` for `"small"` (stress tests), `"max"` (the limits)
  and edge shapes.
- **`solutions/brute.py`:** obviously correct and written differently from the
  references.
- **The four references:** standard library only, read all input at once, write the
  output once.
- **`naive.py` and `naive.rs`** when the complexity matters: correct but slow; the tests
  must make them time out.

`kb-pack validate` refuses any file that still has `TODO(kb-pack)`:

```text
$ cargo run -p kb-pack -- validate
FAIL  content/problems/tidy-shelves
        - editorial.md: still has TODO(kb-pack) markers from `kb-pack new`
        - gen.py: still has TODO(kb-pack) markers from `kb-pack new`
        …
```

### 3. Generate the hidden tests

```sh
cargo run -p kb-pack -- generate content/problems/tidy-shelves --size small --seeds 1,2,3,4,5,6,7,8,9,10,11,12,13,14
cargo run -p kb-pack -- generate content/problems/tidy-shelves --size max --seeds 1,2
```

`gen.py` and `ref.py` run in the sandbox; the outputs come from `ref.py`. The tests are
committed. Each problem's tests must stay within 512 KB compressed (`validate` prints the
size): keep max tests lean, with short values.

### 4. Validate in the sandbox

```sh
cargo run --release -p kb-pack -- validate --run --problems tidy-shelves
```

This checks that `validate.py` accepts every test, every reference is accepted within half
its time limit, the references agree with `brute.py` on 500 generated inputs, and a naive
solution times out. A failing check says what to change.

### 5. Read it as a page

```sh
cargo run -p kb-pack -- preview content/problems/tidy-shelves
```

`target/preview/tidy-shelves.html` shows both statements, the hints and the editorial,
rendered the way Markdown is. Read it once in each language before you open a pull
request.

## A Learn topic, step by step

```sh
cargo run -p kb-pack -- new-topic sorting gnome-sort
```

This writes `content/learn/sorting/gnome-sort/` (the subject must already be in
`subjects.toml`): `topic.toml`, `theory.md` and `theory.es.md` (the same `##` sections),
`sample.py` and `sample.ts` (the same `@step` markers), and `quiz.json`. Then:

1. **The tracer:** add `apps/desktop/src/learn/tracers/gnome-sort.ts`, register it in
   `tracers/index.ts`, and add its narration keys to `learn.narration` in both locales.
2. **The tests:** add a reference implementation and a hand-checked golden run to
   `apps/desktop/src/test/learnReference.ts`.
3. **The cases:** `pnpm learn:cases` writes `cases.json` from the tracer.
4. **The sandbox check:**
   `cargo run -p kb-pack -- validate --learn --run --topics gnome-sort` runs both samples
   on every case and compares them with the tracer.

## A reading exercise, step by step
1. Pick a set under `content/reading/` (or add one: `set.toml` with id, order, kind
   `print` or `complexity`, titles and blurbs in English and Spanish).
2. Write `<n>.py` and `<n>.ts` (at most 30 lines, standard library, no input) and add the
   item to `items.json`: prompts and explanations in both languages, then the answer.
   - Print items: `answer` (or `answer_py` + `answer_ts` when the versions differ on
     purpose) and, usually, 3-4 `choices`.
   - Complexity items: a top-level `f(n)`, `time`, `space` and `dominant` (the costly
     line in each file). Write loops out: builtins such as `sorted` or `in` count as one
     line, and the check counts executed lines.
3. `cargo run -p kb-pack -- validate --learn --run --sets <set>` runs every answer in the
   sandbox (ADR 0035).
4. Optionally link the item from a topic: `reading = ["<set>/<n>"]` in its `topic.toml`.

## A problem pack
Packs carry problems outside the bundled catalogue (ADR 0038). Write a `pack.toml`:

```toml
id = "graphs101"            # 3-20 lowercase letters and digits
name = "Graph drills"
author = "Your name"
version = "1.0.0"
licence = "CC-BY-SA-4.0"
min_app = "1.0.0-beta.1"
```

then `cargo run -p kb-pack -- pack content/problems/a content/problems/b --manifest
pack.toml --out graphs101.kbpack`. Each folder is validated like built-in content
(run `validate --run --problems a,b` first for the sandbox checks); the pack lists every
file's SHA-256. Packs hold problems only (no Learn topics yet). Add
`--assignment assignment.toml` to make an assignment for classroom mode (ADR 0039,
docs/classroom.md); a pack that is only an assignment of built-in problems needs no
problem folders. A pack problem needs one reference solution (built-in problems need all
four); the in-app editor (Settings → Packs → Show the problem editor) writes such
packages from a form (ADR 0040).

## Before the pull request

- `cargo run -p kb-pack -- validate` (structure) and the sandbox check above pass.
- `pnpm test` passes (locale parity, Learn tracers and cases).
- The pull request says what is new and how you checked the statements.

Content is licensed CC BY-SA 4.0, and it must be original: never copy statements or
tests from LeetCode, HackerRank or similar sites.
