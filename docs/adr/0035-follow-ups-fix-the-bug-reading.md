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
