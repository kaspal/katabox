# 40. The in-app problem editor and the Portuguese UI

Date: 2026-11-22 · Status: proposed (expansion M13; the pt-BR part follows in M14)

## Context
Making a pack (ADR 0038) meant writing a problem package by hand: `problem.toml`, two
statements, hints, an editorial, test JSON files, and references that match the
generated stubs exactly. Teachers and clubs need a form, and the app must keep its rule
that the UI never touches the file system.

## Decision: the editor
- **Where:** Settings → Packs → "Show the problem editor" (`developer.create`, a
  machine-only setting outside the export allow-list) shows a link to `/create`, a lazy
  route. (The plan said a Developer section; the Packs section is where packs are made,
  and a new section would add a tab for one switch.)
- **Drafts** live in `<local data>/authoring/<id>/`: `draft.json` (the form) and
  `package/<id>/`, an ordinary function-mode package regenerated from the draft on every
  save (`kb_pack::authoring::write_package`). kb-app writes only there; ids are
  `[a-z0-9]+(-[a-z0-9]+)*`, at most 40 characters (so `<pack>--<id>` fits in 64), and
  never `--`. `draft.json` is read with an 8 MB cap and `deny_unknown_fields`.
- **The form:** id, titles EN/ES, difficulty, pattern (function patterns only), rank,
  tags; the signature (function, parameters, return type) with an "Insert the starter
  code" action that returns the judge's own starter for the reference language (the exact
  declaration the reference must keep); statement, three hint tiers and editorial in EN
  and ES with a live preview rendered as pack text (`Markdown trust="pack"`); public and
  hidden tests, each parameter a JSON field with the M9 visual builder; one reference
  solution in any judge language; checker and limits.
- **Saving** happens shortly after each change and when leaving. Every save returns the
  issues: the draft's own checks (empty texts, at least one hint, sizes, the id), then
  the package validation, each validator message mapped to its form field
  (`authoring::field_of`; unmapped ones show at the top).
- **Pack rules (★):** the editor makes pack problems, which need **one** reference
  solution; built-in content still needs all four (`validate::Rules::Builtin` vs
  `Rules::Pack`, used by the pack import, `kb-pack pack` and the editor). Every other
  rule is the same. A pack problem without a reference in some language simply has no
  custom tests or shrinker in that language (the app already reports "no reference").
  The editor says built-in quality needs all four references and the test scripts.
- **Check in sandbox** judges the reference on every test through the ordinary judge
  path (`App::judge`, the judge slot, the sandbox); it runs only when there are no
  issues. There is no validate.py in an editor package, so only the tests are checked.
- **Export as pack** asks for the manifest (id, name, author, version, licence; min_app
  is this app's version), picks the drafts, writes the `.kbpack` with `write_pack`
  through the Rust Save dialog, then **opens it again exactly as an import would**
  (extract + `check_extracted`); a pack the import would refuse is deleted and the
  reasons are shown.

## Not in the editor
Program and SQL problems, follow-ups, fix-the-bug, gen.py/validate.py/brute.py and
complexity profiles: those stay hand-written packages (docs/problem-authoring.md).

## Consequences
- Tests: `kb-pack` authoring unit tests (a complete draft validates with pack rules and
  builds with one reference, issues map to fields, ids, the starter), kb-app
  `authoring_drafts_issues_and_export` (ids refused, issues, export refused with issues or
  a built-in id, an exported pack that the import previews), `authoring_windows` (Check
  in sandbox: Accepted for a right TypeScript reference, Wrong Answer for a wrong Python
  one), Vitest for the flow and the a11y pass.
