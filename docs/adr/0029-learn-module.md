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
- **Not added yet:** `d3-hierarchy` and `d3-force`, until the tree and graph phases
  need them.
- **Reused:** code highlighting uses the app's Lezer highlighter (styles/syntax.css).

## Consequences
- **A new topic needs four things:**
  - a package in `content/learn`;
  - a tracer in `src/learn/tracers` with its narration keys in both locales;
  - `pnpm learn:cases`;
  - `kb-pack validate --learn --run`.
- **The sample check wraps the TypeScript sample with the driver's `import` first.**
  That's because `node --check` only strips types from a `.ts` file that starts like an
  ES module.
  - The same quirk affects program-mode TypeScript solutions that have type annotations
    and no leading `import`: they get a false compilation error.
  - That's recorded for a separate fix.
- **Paths through topics are left for Phase 6.**
