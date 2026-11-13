# 27. Debugger snapshots for the visualizer

Date: 2026-11-13 · Status: accepted (maintainer sign-off on the data path)

## Context
- **The visualizer draws the debugger's variables as pictures:** arrays with pointers,
  grids, linked lists, trees, maps and graphs. It updates on every step, forward and
  back.
- **The debugger's values are text** (ADR 0022): each variable is a `repr` cut to 200
  characters plus a child count. Children arrive only through expansion, and every
  expansion is another stop replay in the sandbox (0.3-2.7 s). A list of 30 nodes would
  need dozens of replays per step.
- **Each step is a separate process** (replay), so object addresses mean nothing from one
  step to the next.

## Decision
### A structured snapshot in the stop record
- **The request:** a stop request may name variables, `visualize: [name]` (at most 16).
- **What the harness adds:** its stop record gains a `snapshot` key, built after the
  stop, right before the process exits:
  - Python: `describe_stop`, then `os._exit`;
  - JS/TS: `describeStop`, then `process.exit`.
- **Consequences:**
  - It can never change the replay, so there is no divergence risk.
  - It costs no extra sandbox run: one stop run per step change, as before.
  - Without `visualize`, nothing is added: the request, the record and the cost are
    exactly as before.
- **The session keeps it:** kb-app's session stores `visualize` with the frame,
  expansions and watches (`DebugInspect`), so every action (step, back, continue,
  restart) and every inspect sends it. The UI sends it only while the Visualize view is
  open.

### Format (`kb_core::debug::DebugSnapshot`)
```
DebugSnapshot { roots: [SnapRoot], objects: [SnapObject], truncated, error? }
SnapRoot   { name, value: SnapValue }
SnapValue  = {t:"ref", id} | {t:"int"|"float"|"str"|"bool"|"none", v: text} | {t:"opaque", v: type name}
SnapObject = {kind:"list"|"set", type, items, len} | {kind:"map", type, entries: [[k, v]], len}
           | {kind:"list_node", val, next, input?} | {kind:"tree_node", val, left, right, input?}
```
- **`roots`:** the requested variables found in the selected frame (locals, then module
  globals), followed by every other local holding a ListNode / TreeNode, so pointers
  like `slow` and `fast` are labelled.
- **Scalars are display text,** so big integers never lose digits on the way to
  JavaScript.
- **`len`** is always the real length; `items` / `entries` may list fewer.

### Identity
- **Within a snapshot,** each object is numbered once: a breadth-first walk over the
  roots in order, with a visited map keyed on Python `id()` or a JS `Map`. A shared node
  or an alias is a second `ref` to the same id, and a cycle is a `ref` back.
- **Across steps,** the input's nodes keep a stable `input` number:
  - the harness numbers them when it builds them (`from_json` / `fromJson`), in a fixed
    walk of the arguments, only when `visualize` is set;
  - it keeps references to them, so a dropped node's address is never reused;
  - nothing is written onto user objects.
- **Nodes the solution creates** have no `input`, so the UI's change highlighting
  compares them by position (v1).

### The snapshot runs no user code
- **Exact built-in containers only:**
  - Python: `list`, `tuple`, `deque`, the `dict` family (`dict`, `defaultdict`,
    `Counter`, `OrderedDict`), `set`, `frozenset`;
  - JS: arrays, `Map`, `Set`, plain objects (prototype `Object.prototype` or `null`).
- **ListNode / TreeNode fields are read without triggering user code:**
  - Python reads the instance `__dict__` through `object.__getattribute__` (no
    properties, no `__getattr__`); a `__slots__` class is opaque.
  - JS reads only own data properties, through `Object.getOwnPropertyDescriptor` (no
    getters). An array index that is an accessor shows as `opaque "accessor"`.
- **JS Proxies are opaque.** The Proxy check comes first, because a Proxy of an array
  passes `Array.isArray`.
- **Scalars are formatted by the harness:**
  - Python `int` uses `int.__repr__`. Past 64 characters it shows its first digits and
    its digit count; past 13 000 bits it shows `≈2^bits`.
  - JS `bigint` uses a template string, with the same digit-count rule.
  - Strings are cut to 64 characters.
  - Anything else is `opaque` with its type name (read without `repr` or `toString`).
- **The built-ins are saved before the solution loads:**
  - Python: `id`, `type`, `len`, the container types, `dict.items`,
    `OrderedDict.items`, `object.__getattribute__`, `json.dumps`, …;
  - JS: `Reflect.apply`, `Object.getOwnPropertyDescriptor`, the Map and Set iterator
    methods, `util.types` brand checks, `Buffer.byteLength`, …

  Every call goes through those saved references, so a solution that patches `builtins`
  or a prototype can't change what the snapshot sees.
- **JS serialisation:** the JS snapshot is built from null-prototype objects and
  serialised by its own small encoder (no `toJSON`). It is returned to the recorder
  thread as one string (one `Runtime.callFunctionOn` with `returnByValue`), and the
  recorder parses it in its own realm.
- **A failure never breaks the stop.** Any exception gives
  `{error: "snapshot failed: <type>"}`, and kb-judge turns a shape it doesn't know into
  `{error: "invalid"}`.

### Caps
| Cap | Value |
| --- | --- |
| Objects (containers and nodes) | 1 000 |
| Items or entries per container | 500 |
| Scalar values | 10 000 |
| Depth | 32 |
| Characters per string | 64 |
| Requested names | 16 |
| Estimated size (soft) | 200 000 bytes |
| Hard size (`SNAPSHOT_MAX_BYTES`) | 256 KiB |

- **Hitting a cap** stops adding and sets `truncated`; the UI says "showing first N of
  M". An object cut off by depth or by the object cap is `opaque`.
- **The hard size is checked twice:** in the harness, on compact JSON, and in kb-judge.
  Over it, the snapshot is `{error: "too_large"}`.
- **The value cap was planned at 20 000.** It became 10 000, plus the soft size estimate,
  because 20 000 values of about 20 bytes each would pass the hard size: the snapshot
  would have been dropped instead of cut.

### Output caps and cost
- **The sandbox's output cap is unchanged:** `TRACE_BUDGET_BYTES + RECORD_ALLOWANCE +
  2·print_cap`, about 1.66 MB. A stop run prints no trace and no prints, so a snapshot
  of at most 256 KiB fits inside the 512 KiB `RECORD_ALLOWANCE` with the locals,
  expansions and watches.
- **Measured** with `visualize_windows`, release build, a 10⁵-element input, a 100 × 100
  grid and a 5 000-entry dict, at the caps (median of three stops):

  | Language | Stop alone | Snapshot overhead | Snapshot size |
  | --- | --- | --- | --- |
  | Python | about 250 ms | 6-85 ms | 166 KB |
  | TypeScript | about 230 ms | 23-40 ms | 164 KB |

  The target is under 150 ms; the test asserts under 300 ms so a busy machine doesn't
  fail it.

### Nothing else changes
There is no change to kb-runner, the sandbox, its limits, the CSP or the capabilities.

### The Visualize tab (UI)
- **Placement:** a tab next to Variables in the Debug panel, plus an "Open larger" dialog
  with its own step controls and zoom. While the tab is open, `visualize` is sent with
  every stop; closing the tab sends one inspect without it.
- **Drawing:** hand-written SVG in `workspace/debug/visualize/`. A pure `model.ts` detects
  the shapes, `layout.ts` places the nodes (an in-order tree, a circle for graphs) and
  `diff.ts` finds the changes. There is no new dependency.
- **Shapes:** arrays, sets, grids (lists of equal lists of scalars), linked lists (a
  cycle is drawn once, with an arc back), trees, and maps (as a table). A graph (an
  adjacency list, a 0/1 matrix or a dict of lists) or a heap (an array as a tree) is
  drawn only when the user picks it.
- **Graph limit:** at most 50 nodes, with "showing first N". Duplicate edges are merged
  with a count, two-way edges are drawn as one line with two arrowheads, and self-loops
  are kept.
- **Pointers:** int locals with the usual names (`i`, `j`, `lo`, `hi`, `left`, `right`,
  `mid`, …) point into every drawn array they fit, from 0 to `len` ("end"). Grid
  pointers come from (`r`, `c`), (`row`, `col`) and (`i`, `j`). Nodes are labelled with
  the other variables that point at them.
- **Per-problem choice:** the setting `workspace.visualize.<problem>` (importable) holds
  the variables, the view per variable, and pointer overrides, which work both ways. The
  default is the problem's parameters.
- **Changes since the previous step:** each change is marked with a dashed outline and a
  "•", never by colour alone. Nodes from the input are compared by their `input` number
  across every drawing of the previous step, so a node that moves from `back` to `prev`
  in a reversal is still marked. Other nodes are compared by position within their own
  drawing.
- **Tokens:** `--viz-node`, `--viz-edge`, `--viz-pointer` and `--viz-changed` in light
  and dark, covered by the contrast tests.

## Consequences
- **The Visualize view gets a picture on every step** for the price of the stop it
  already makes.
- **Nodes the solution allocates** have no stable identity across steps, so change
  highlighting for them is positional. Tracking constructors could add it later.
- **The two harnesses carry parallel snapshot code** (about 200 lines each), with the
  same caps. `visualize_windows` tests both on the same cases: real problems, identity
  and cycles, caps and truncation, no user code with patched built-ins, and the cost.
