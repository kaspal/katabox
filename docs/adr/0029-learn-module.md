# 29. Learn module (algorithm visualizer)

Date: 2026-11-14 · Status: accepted (maintainer sign-off on the section, `motion`, the content type with schema 6, and the TypeScript tracer exception)

## Context
- **The maintainer planned a Learn section:** about 114 algorithm topics in 13 subjects,
  delivered in 7 phases. Each topic has the theory, a tested sample in Python and
  TypeScript, and an animation the user can play, step and scrub.
- **Phase 0** builds the foundation every later phase reuses:
  - the section and its pages;
  - the step engine, the player and two scenes;
  - the topic package format, built into content.db;
  - the correctness checks;
  - three topics: Bubble Sort, Insertion Sort and Quick Sort (Lomuto).

  It ends at a look-and-feel review before more topics are written.
- **The design was approved** as mockups (the topic page, Learn home, the dark theme and
  a DP table).

## Decision
### The section
- **Routes:**
  - `/learn` is the home: search, Continue, and the subject grid. Subjects with no
    topics yet say "Coming soon" and are not links.
  - `/learn/$subject/$topic` is the topic page: the topic picker, the stage and the
    Theory / Code / Practice tabs.
  - The nav entry sits between Library and Interview, and the command palette lists
    every topic.
- **Both routes are lazy.** The engine, the scenes, the tracers and `motion` load only
  when a Learn page opens.
- **Startup cost:** it grows only by the Learn strings in the bundled locales and a few
  CSS classes. The report records the measurements.

### Tracers are TypeScript teaching content (the exception)
- **Where they live:** a tracer is a pure function from an input to every step
  (`Step[]`), in `apps/desktop/src/learn/tracers/<topic>.ts`.
- **This is the one exception** to "business logic goes in `crates/`".
  - Tracers are teaching content: they exist to drive an animation, and the UI needs
    every step up front to scrub instantly.
  - They never touch I/O, the judge or user data.
- **Why not content:**
  - Running TypeScript from content.db would need `eval` or a code-loading build step,
    and the CSP forbids that.
  - As app code, tracers are type-checked, tested by Vitest and code-split.
- **Checks:**
  - kb-pack checks that every topic has a tracer file.
  - Vitest checks that the registry matches the files.

### Steps
- **Each `Step` is a snapshot,** not a diff. It holds:
  - the items, where each value keeps a stable `id` so scenes can key on it and equal
    values still slide;
  - the roles by position;
  - the lifted position (insertion's key), the pointers and the active range;
  - the counters (comparisons, swaps or shifts, array writes);
  - a narration key with parameters;
  - the `@step` marker of the sample line behind the step;
  - the variables.
- **Snapshots make scrubbing O(1).** At Phase 0 sizes (16 values, a few hundred steps)
  they cost nothing. A later phase may add diffs for large graphs.
- **No English inside tracers.** Narration is an i18n key under `learn.narration.<topic>`
  with parameters. The key type comes from the English locale, so a missing key is a
  compile error, and Vitest checks EN/ES parity.
- **Caps:** inputs are checked against the topic's limits before the tracer runs.
  `MAX_STEPS` (2000) caps a trace, and a breach shows a clear message.

### The player
- **One timer at a time,** scheduled against a target time, so playback doesn't drift.
  The timer is cleared on pause, unmount, a new trace and a speed change.
- **Speeds:** 0.5, 1, 2 and 4×.
- **Restart:** Play at the end starts over.
- **Timeline:**
  - while the timeline is dragged, scenes jump with no transition;
  - stepping faster than a transition retargets the running spring (`motion`), so
    animations never queue.
- **Shortcuts on the topic page:** Space, ←/→ and Home.
  - The page handler ignores text entry, sliders, selects, tabs, menus and dialogs.
  - For Space and Enter it also ignores buttons and links, which activate natively. A
    focused Play button therefore toggles once.
- **"Reduce motion"** turns every scene transition off; stepping still works.

### "Viewed"
- **The rule:** a topic counts as viewed once its animation has reached the final step
  **by playing or stepping**.
- **What doesn't count:** opening the page, or scrubbing straight to the end.

### Per-viewer state (user.db settings, no migration)
- **The keys:**
  - `learn.viewed`: the viewed topic ids;
  - `learn.last`: `{subject, topic, step, total}`, for Continue;
  - `learn.speed`;
  - `learn.layout`.
- **Export:** `learn.viewed` and `learn.last` travel with an export (the import
  allow-list in `kb-app/src/transfer.rs`). Speed and layout stay on the machine.
- **Exports now skip any setting an import would refuse.** Before, such a setting made
  every import of that file fail.

### Scenes
- **`BarsScene`** (sorting) and **`ArrayScene`** (cells, with a lifted key over a dashed
  gap).
- **How they're drawn:**
  - SVG, with `motion` springs for positions and lift;
  - CSS transitions for colours (motion can't tween `var(--…)` colours);
  - themed only by `--viz-*` tokens.
- **Contrast:** roles are at least 3:1 on the card, and pointer labels and the code
  line read as text (at least 4.5:1), in both themes. `out` is a deliberate fade, and
  the range band and the text fallback carry the same information.
- **Colour is never the only cue:**
  - compared bars get a dashed outline and moving bars a solid one;
  - active bars lift;
  - pointers are labelled;
  - the legend names every role.
- **Accessibility:** each scene is `role="img"` with a one-sentence summary, plus a
  visually hidden list of positions and roles. The narration is an `aria-live` region.
- **Palette change:** the mockup's light-theme amber, used for compared bars (`#F59E0B`,
  2.2:1), is darkened to `#D97706`, and the swap orange to `#C2410C`, to keep 3:1 and
  keep the two apart.

### Topic packages and content.db (schema 6)
- **Layout:** `content/learn/subjects.toml` (13 subjects: id, order, EN/ES names and
  blurbs, an icon name from the UI's map), then one folder per topic,
  `content/learn/<subject>/<topic>/`, with:
  - `topic.toml`: id, subject, order, EN/ES titles, tags, complexity, linked problems,
    presets, the default input, the input limits, and the call the sample check makes;
  - `theory.md` and `theory.es.md`, with the same `##` sections, and
    `learn:<subject>/<topic>` links to other topics;
  - `sample.py` and `sample.ts`;
  - `quiz.json`: 2-5 "predict the next step" questions, both languages;
  - `cases.json`: inputs and the tracer's final state.
- **Step markers:**
  - samples carry `# @step <id>` / `// @step <id>` as a trailing comment on the line they
    mark;
  - the UI strips them and maps id → line, without renumbering;
  - kb-pack checks the markers are well formed, unique, and the same set in both
    samples;
  - Vitest checks the tracer emits exactly those markers.
- **The content.db tables:**
  - `learn_subjects`, `learn_topics`, `learn_texts`;
  - `learn_samples` (raw, markers included);
  - `learn_quiz` (both languages, localized at read time);
  - `learn_fts`, FTS5 over title, tags and theory in both languages, with the
    `problem_fts` tokenizer.
- **`CONTENT_SCHEMA_VERSION` goes from 5 to 6.**
- **Commands:** `learn_catalog`, `learn_topic` (with the linked problems' summaries) and
  `search_learn`.

### The correctness guarantee
- **Where `cases.json` comes from:**
  - `pnpm learn:cases` writes it from the tracers: the four presets plus 20 seeded
    random inputs;
  - Vitest fails when it's stale (`toMatchFileSnapshot`);
  - kb-pack checks each final state is the sorted input, for sorting topics.
- **What `kb-pack validate --run` does:**
  - it runs both samples on every case in the sandbox through the **program-mode judge**
    (ADR 0028), wrapping each sample in `languages/<lang>/learn_driver.*`, which reads
    the values, makes the topic's call and prints the array;
  - the `tokens` checker compares the output with the tracer's final state;
  - `--learn` limits validation to the topics;
  - `kb-pack/tests/learn_samples_windows.rs` does the same, and also shows that a wrong
    sample is caught.
- **No kb-runner change.** Samples run as ordinary sandboxed programs.

### Dependencies
- **`motion`** (motion.dev, MIT, pinned to 14.8.0), only in the lazy Learn chunk.
- **`d3-hierarchy` 3.1.2 and `d3-force` 3.0.0** (ISC, exact pins), added in Phases 2-4
  for tree layouts and graph layouts; only in the lazy Learn chunk.
- **Reused:** code highlighting uses the app's Lezer highlighter (styles/syntax.css).

### Phase 1 additions (Foundations, Sorting, Searching, compare mode)
- **Steps carry optional scene data:**
  - extra rows: `aux` (prefix sums, a merge buffer, a resized array's storage);
  - `buckets`, `stack`, `tree` (positions fixed from the final tree), `pegs`, `bits`;
  - `chart`, `capacity`, `window`, merge `runs`, `heap` size.
- **`hidden` ids keep a value's identity across rows:** a value in a merge buffer or a
  bucket is drawn only there, and its main-row slot shows empty. `items` stays a
  permutation, so the final row is still the sorted input.
- **`result` on the last step is what the sample returns** when that isn't the final row
  (an index, counts, a list of moves). `cases.json` output comes from it, so it is always
  the tracer's own answer. Vitest also compares every output with an independent
  reference (`src/test/learnReference.ts`): brute force or closed forms, never the
  tracer's code.
- **Input kinds**, in the TypeScript tracer definition, not in content.db (no schema
  change):
  - `values`;
  - `target`: `[target, ...values]`;
  - `sorted-target`: the same, with the values ascending;
  - `count`: `[n]`.

  The flat list keeps the DB, the presets and the sample driver unchanged. The Stage
  adds a Target or n field, and refuses unsorted values or a target out of range with an
  inline error (never by silently fixing the input).
- **The driver's call may set `a` to the sample's result**, e.g.
  `a = [binary_search(a[1:], a[0])]`. TypeScript's driver declares `let a`.
- **`sorts` in topic.toml:**
  - it defaults to true in `sorting`;
  - it switches the "output is the sorted input" check;
  - quickselect sets it false.
- **Scoped sandbox runs:**
  - `kb-pack validate --learn --run --topics a,b` or `--subject s` (an unknown id
    fails);
  - `KB_LEARN_TOPICS=a,b` scopes `learn_samples_windows`.
- **Compare mode:**
  - **Who can be compared:** topics of one subject with the same `compareGroup` (`sort`:
    values to sort; `find`: a value to look for), on an input that fits both.
  - **The view:** two panes with their own legend, counters and narration, under one
    player and one input row. The player is as long as the longer run, and the shorter
    run holds its last step, marked "Finished".
  - **Not viewing:** compare mode never marks a topic viewed or saves the Continue
    position, and it adds no setting.

### Phases 2-6 additions (data structures, graphs, grids, DP, greedy, backtracking, strings, math)
- **New scenes**, one per data shape: list, slots, tree (d3-hierarchy, with optional
  extra `links` for tries and automata), hash, bit array, graph (d3-force, positions
  fixed once per input), grid, DP table, timeline, decision outline, board, string rows
  and plane. Each draws only `--viz-*` tokens and gives the stage a text summary.
- **Inputs stay a flat number list in content.db.** Richer inputs are codecs in the
  TypeScript tracer definition (`engine/codecs.ts`, `graph.ts`, `grid.ts`, `shapes.ts`):
  operations, text, graphs `[n, directed, u, v, w…]`, map-style grids, intervals,
  items, matrices, points. The language drivers decode them with matching helpers
  (`kb_ops`, `kb_text`, `kb_graph`, `kb_grid`, …), so samples read natural input.
- **Limits keep animations short:** every preset and random case must pass the input
  check, and a worst-case input per topic stays under half the step cap (Vitest).
  Grids are capped at 12×16, bitmask TSP at 5 cities.
- **The `scc` topic uses Kosaraju** (two DFS passes); its theory names Tarjan as the
  alternative.
- **Learning paths (content.db schema 7):** `content/learn/paths.toml` lists ordered
  topic ids with titles and blurbs in both languages; kb-pack checks ids, texts and
  known topics, and the build writes them to a `learn_paths` table. Learn home shows each
  path's progress (topics viewed) and links to the first unviewed topic. Progress comes
  from the existing `learn.viewed` setting, so there is no new user data.
- **"See it animated":** topic summaries carry their linked problems, and a problem's
  Editorial tab lists the topics that link to it. The links name the technique, so they
  show only once the editorial is unlocked (and never in review or mock interviews).
- **Terms:** `docs/learn-glossary.md` fixes the English-Spanish vocabulary.

## Consequences
- **A new topic needs four things:**
  - a package in `content/learn`;
  - a tracer in `src/learn/tracers` with its narration keys in both locales;
  - `pnpm learn:cases`;
  - `kb-pack validate --learn --run`.
- **The sample check wraps the TypeScript sample with the driver's `import` first.**
  This was first a workaround for `node --check`, which did not strip types from a `.ts`
  file without a leading `import`. The program-mode check no longer uses `node --check`
  (ADR 0028, `fix/ts-program-check`), so the import's position no longer matters; it
  stays first.
- **A new path is a `paths.toml` entry**; kb-pack refuses unknown or repeated topics.

## Addendum (2026-11, features-content M9): the Databases subject
- **Topics:** join algorithms (a hash join), GROUP BY (hash aggregation), window
  functions (running totals per partition) and recursive queries (WITH RECURSIVE, round
  by round).
- **The relations scene:** input tables, any helper table (the hash index, the groups),
  and the result growing below. Rows use the usual roles, plus a dashed or solid outline
  so colour is never the only cue.
  - It is drawn as **real HTML tables** inside a labelled `role="group"`, not an
    `<svg role="img">`, so screen readers read rows and columns as a table.
  - The every-scene test accepts either form. It still requires the summary and the text
    fallback to describe every step.
- **Inputs:** `rowsCodec` (one or two tables of two-column rows) and `bossesCodec` (a
  boss list without loops).
- **Samples:** each `sample.py` checks its own answer against SQLite in memory before
  returning (`sqlite3` is in the pinned Python). So every sandbox run of the sample also
  proves that SQLite agrees with the animation. The TypeScript samples have no database
  and only compute.
