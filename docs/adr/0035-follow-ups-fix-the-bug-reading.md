# 35. New exercise types: follow-ups, fix the bug, reading exercises

Date: 2026-11-22 · Status: proposed (expansion M5-M7)

## Context
Practice so far was one kind of exercise: write a solution from a statement. Common
practice formats have more:
- a harder sibling of a problem you have just solved;
- buggy code to repair;
- code to read.

## Decision: follow-up variants (M5)
**A follow-up is a full problem package** at `content/problems/<base>/followups/<n>/`.
- `problem.toml` gains `parent = "<base id>"`, and the id is `<base>--f<n>`.
- Its statement is the addendum ("what changes"), and it has its own hints, editorial,
  tests, references in all four languages, `brute.py`, `gen.py` and `validate.py`.
- Every existing rule and check applies unchanged: structure, the 512 KB budget, 2⁵³
  integers, references within 50% of the cap, and the stress test against `brute.py`.
- The base package is never touched.

**kb-pack:**
- `problem_dirs` lists each base followed by its follow-ups;
- `problem_id_of` names a folder;
- `--problems <base>` selects the base with its follow-ups, and `--problems <base>--f<n>`
  selects one.

**Validation rules:**
- `parent` is exactly the base folder, the folder name is a number from 1 to 9, and the
  pattern and mode are the base's.
- `parent` appears only on follow-ups.
- **Base ids never contain `--`.** That namespace is kept for follow-ups and packs
  (ADR 0038).

**content.db schema 9:**
- `problems.parent_id`, with an index.
- `list_problems` leaves follow-ups out, and so do daily, next problem, katas, plans and
  ratings, which all build on that list.
- `ProblemView` carries `parent` and `followups` (id, title, difficulty, unlocked).
- A pack whose follow-up has no base is refused.

**App:**
- A follow-up is unlocked once its base is solved. This is a UI convenience, not a
  security rule: every problem can be judged as usual.
- The workspace shows "Base problem · Follow-up 1 · …" (locked ones with a lock and a
  tooltip).
- A follow-up's page has an ExerciseShell banner ("A harder follow-up of <base>", k of n).
- Submissions, drafts, status, the journal and export all work through the normal ids:
  no user.db change.

**Content:** 40 follow-ups on 30 base problems.
- Each idea is enforceable by tests and limits (no "O(1) memory" follow-ups).
- Each was checked against the base so it doesn't repeat it.
- JS-safe numbers (A1): `path-tiles--f1` caps n at 10¹⁵ and uses the prime 1 000 003, so
  every product stays below 2⁵³.

## Decision: fix the bug (M6)
**A kind, not a mode.** `kind = "fix"` in problem.toml, orthogonal to `mode`. Fix
problems are judged exactly like function problems; a new `ProblemMode` would have
rippled through every judge branch for nothing. The kind changes only the starter code,
a badge, a Library filter and practice selection.

**Package:** an ordinary function package plus:
- `solutions/buggy.py` and `solutions/buggy.ts` (required), `buggy.php` and `buggy.rs`
  (optional);
- each buggy file contains the stub's declaration line, like the references;
- the statement opens with a short story about the code that fails.

kb-pack refuses buggy files without `kind = "fix"`, an unknown kind, a fix problem in
program or SQL mode, and a fix follow-up.

**`validate --run`:** each buggy file is judged on every test and must **not** be
Accepted; the output names the first test that catches it (`buggy.py ok caught: test 17
…`). The references must pass, as for every problem.

**content.db 10:**
- `problems.kind TEXT NOT NULL DEFAULT 'solve' CHECK (kind IN ('solve','fix'))`;
- `starters(problem_id, language, code)`, the buggy code per language.
- `ProblemSummary.fix`, `ProblemView.fix` and `ProblemView.fix_languages`.

**App:**
- `get_stub` returns the buggy code when there is one for that language, otherwise the
  ordinary starter; the banner then says "no buggy version in this language".
- **Reset to the buggy code** (with a confirmation) replaces the draft; snapshots stay.
- The daily problem, next problem, placement and study plans leave fix problems out
  (`practice::without_fix`): they suggest problems you solve from scratch. Katas, lists,
  interviews and the rating treat them as ordinary problems.
- The shrinker, debugger and custom tests work unchanged.

**Content:** 24 problems (8 Easy, 11 Medium, 5 Hard), one classic bug each: an
off-by-one range, integer division, a shadowed counter, the last run never compared,
`lo < hi` missing the last slot, removing while iterating, a wrong tie-break, a
non-empty stack accepted, overflow (A2: the inputs stay JS-safe, the TypeScript fix needs
BigInt, and the hints say so), the best length recorded before the window is repaired, a wrong recursion base, the start
never marked visited, sorting by end, duplicate pairs counted twice, a memo key missing
a parameter, a min-heap instead of a max-heap, a prefix index off by one, aliased rows,
money in floats, Dijkstra finalising on push, linking nodes instead of roots,
`bisect_right` in a strict LIS, a choice never undone, and recursion too deep for a
5·10⁴ chain (A2: buggy only in Python and TypeScript; no tree is deeper than 1000, the
chain is a parent array).
