# 36. Stress lab and visual test builders

Date: 2026-11-22 · Status: proposed (expansion M8-M9)

## Context
When a solution fails a hidden test, or passes the public ones but feels shaky, the
classic move is a stress test: write an obviously correct brute force, run both on many
small random inputs, and look at the first input where they disagree. Doing that by hand
means a generator, a loop and a diff, outside the app.

## Decision: stress lab (M8)
**What runs:** the user's solution and the user's own brute force, both through the
ordinary judge path, in the sandbox, one sandbox user at a time. The run holds the judge
slot, so it never overlaps a Run, Submit or shrink. No kb-runner change, no new
capability.

**Inputs:**
- With the Python runtime: the problem's `gen.py` (now in content.db 12 as
  `problems.generator`) with seeds and the `"small"` size, in the sandbox, then its
  `validate.py`. These are the same scripts kb-pack's own stress check uses.
- Without it: `kb_judge::randgen`, a pure, seeded generator from the signature (arrays of
  0-8, ints in −20..20, strings over "abc", level-order trees of 0-7 nodes, lists of 0-8).
  These inputs can't be checked against the constraints (validate.py needs Python); the
  UI says so.
- Program problems need gen.py, so the Python runtime. SQL problems have no stress lab.

**Loop** (`kb-app/src/stress.rs`, modelled on the shrinker of ADR 0030), per round of up
to 16 inputs:
1. the brute force runs as custom tests (expected unknown); its outputs become the
   expected outputs;
2. the solution runs through the problem's checker;
3. the first disagreement stops the run with the input, both outputs and the verdict.

A brute force that crashes, times out or doesn't compile stops the run as "your brute
force failed", never as a disagreement. Budget: 40 rounds or 60 s; Stop is checked before
every sandbox run. Progress events go over a Channel.

**Brute forces are kept** per problem and language (user.db 0010 `stress_brutes`,
exported; newer `updated_at` wins on import), at most 64 KB.

**UI:** a **Stress test** tab in the results pane: the brute-force editor (CodeMirror,
inside the lazy workspace route), Start/Stop, progress, and on a disagreement the input,
both outputs, **Debug this case**, **Add to custom tests** and (function problems)
**Find a smaller failing input**. Not locked in interviews or katas, like the shrinker.

**Tests:** randgen properties (determinism, sizes, types, trimmed trees); a Windows e2e
(`kb-app/tests/stress_windows.rs`) where seeded wrong solutions disagree with a correct
brute force in Python, TypeScript and a program problem, and a crashing brute force is
reported as such. The attack suites ran with this milestone.

## Decision: visual custom-test builders (M9)
**One source of truth:** each custom test's parameter keeps its JSON text field. A
**Build visually** button under the field opens a builder that reads that text and writes
JSON text back; when the text doesn't parse (or has the wrong shape), the builder says to
fix it as JSON instead of guessing.

**Which builder** (`workspace/results/builders/model.ts`, pure and tested):
- `TreeNode` → a tree: click a node to change its value, add a left or right child, or
  remove it with its subtree (canonical level order with nulls, ADR 0025);
- `ListNode` and other one-dimensional arrays → rows: edit, add, remove and reorder;
- `string[]` → a grid when the rows have equal length (cells cycle through the
  characters in the grid plus `.` and `#`), else rows;
- `int[][]` → a graph when every row has 2 or 3 numbers and the parameter's name looks
  like an edge list (`edge|link|road|pair|merge|door|friend|connection`), else rows; a
  **View as** switch offers the other one. The graph layout is the Learn engine's
  force layout; click one node, then another, to add an edge (weight 1 when the edges are
  weighted); weights and removals are in the edge list.

**Keyboard:** every action has a keyboard path: nodes and cells are buttons (Enter),
Delete removes a focused tree node, arrow keys move between grid cells, and the graph's
edge list has inputs and remove buttons.

**Check:** a **Check inputs** button runs `check_custom` (kb-app): the signature's types,
then the problem's `validate.py` in the sandbox (one run, holding the judge slot), and
shows each input's broken constraint. No solution runs. Without Python only the types
are checked, and it says so. Function problems only (program inputs are free text).
