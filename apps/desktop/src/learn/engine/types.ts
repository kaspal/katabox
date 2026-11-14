// The Learn engine's step format (ADR 0029). A tracer turns an input into every step up
// front, so playing, stepping and scrubbing only index into an array.
import type en from "@/i18n/locales/en.json";
import type { LearnInput } from "@/bindings";
import type { InputError } from "./inputs";

/** A value with a stable identity: scenes key on `id`, so equal values still slide. */
export type Item = { id: number; value: number; /** Shown instead of the value (a letter). */ text?: string };

/** How a position is drawn. Every role has a legend label; colour is never the only cue. */
export type Role = "idle" | "out" | "pivot" | "compare" | "swap" | "sorted" | "key";

export type PointerTone = "pivot" | "i" | "j" | "key";
export type Pointer = { label: string; at: number; tone: PointerTone };

/** Three counters per step; what each one counts is the topic's (`TracerDef.counters`). */
export type Counters = { comparisons: number; moves: number; writes: number };

type Locale = (typeof en)["learn"];
type Narrations = Locale["narration"];
export type TopicId = keyof Narrations;
/** A narration key of one topic, typed from the English locale. */
export type NarrationKey<T extends TopicId> = `${T}.${keyof Narrations[T] & string}`;
export type Narration = { key: string; params?: Record<string, string | number> };
/** A counter label (`learn.stage.<label>`). */
export type CounterLabel = keyof Locale["stage"]["counter"];
/** A legend label (`learn.roles.<label>`). */
export type RoleLabel = keyof Locale["roles"];
/** A row label for an extra row of cells (`learn.stage.rows.<label>`). */
export type RowLabel = keyof Locale["stage"]["rows"];
/** A translated note under a tree node (`learn.stage.notes.<key>`), written "@key". */
export type NoteKey = keyof Locale["stage"]["notes"];

/** A variable shown in the variables grid. */
export type VarValue = number | boolean | string | null;

/** A second row of cells under the main row: a merge buffer, prefix sums, a resized
 * array's new storage. `null` cells are empty slots; `offset` shifts the row right by
 * that many slots (prefix sums start half a slot before the values). */
export type AuxRow = {
  label: RowLabel;
  cells: (Item | null)[];
  roles?: Record<number, Role>;
  offset?: number;
};

/** One bucket of the buckets scene (counting, radix, bucket sort). */
export type Bucket = { label: string; items: Item[] };

/** A call-stack frame or a stack entry: text such as `sum(0, 7)`, newest last. */
export type StackEntry = { id: number; text: string; role?: Role; item?: Item };

/** A node of a recursion tree (parent before child), with what it returned once known. */
export type TreeNode = {
  id: number;
  parent: number | null;
  text: string;
  role: Role;
  result?: string;
  /** Column and depth in the final tree (`placeTrees`), so nodes never move. */
  x?: number;
  depth?: number;
};

/** One row of bits (most significant first when drawn), with its operator label. */
export type BitRow = {
  label: string;
  value: number;
  role?: Role;
  /** The row is the one above shifted: -1 left, 1 right (it slides in from there). */
  shift?: -1 | 1;
};

/** Optional data for scenes beyond the main row. Snapshots, like the rest of the step. */
export type SceneData = {
  aux?: AuxRow;
  buckets?: Bucket[];
  /** The bucket being filled or emptied, or null. */
  bucket?: number | null;
  stack?: StackEntry[];
  tree?: TreeNode[];
  /** The final tree's size: leaf columns and depth (`placeTrees`). */
  treeSize?: { columns: number; depth: number };
  /** Hanoi: disk sizes on each peg, bottom first. */
  pegs?: number[][];
  /** The value being acted on outside the main row (Hanoi's moving disk), or null. */
  active?: number | null;
  bits?: BitRow[];
  /** Highlighted window (inclusive positions) that does not fade the rest. */
  window?: [number, number] | null;
  /** Heap sort: positions 0..heap-1 are the heap drawn as a tree. */
  heap?: number;
  /** Merge view: the two runs being merged (inclusive positions). */
  runs?: [[number, number], [number, number]] | null;
  /** Growth chart: how many curves are shown, and the input n. */
  chart?: { shown: number; n: number };
  /** Dynamic array: the backing capacity. */
  capacity?: number;
  /** Ids drawn somewhere else this step (in the merge buffer or a bucket), so their main-row
   * slot shows as empty. `items` stays a permutation; when set, `aux` and bucket cells
   * share identity with the main row, so a value slides between rows. */
  hidden?: number[];
  /** Buckets scene: the most values any bucket holds during the trace (fixed height). */
  bucketDepth?: number;
  /** Stack scenes: the deepest the stack gets during the trace (fixed height). */
  stackDepth?: number;
  /** Linked list: nodes in display order and their links by node id. `fixed` keeps the
   * nodes where they are (a reversal rewires arrows, nodes stay put). */
  list?: ListData;
  /** Slots of a stack (bottom first), a queue or deque (front first) or a ring buffer
   * (by index), with pointers on slot positions. */
  slots?: SlotsData;
  /** A key → value table beside the scene (LRU's map). */
  table?: TableData;
  /** Trees and forests (BSTs, tries, union-find): drawn with a tidy layout. */
  forest?: ForestNode[];
  /** A hash table: buckets (chaining) or one item per slot (open addressing), the slots
   * visited for the current key, and how its home slot was computed. */
  hash?: HashData;
  /** A bit array (Bloom filter) with the positions the current key touches. */
  bitArray?: { bits: number[]; hits: number[]; miss?: number | null; key?: number | null };
  /** A graph (Phases 4-5): fixed node positions, roles on nodes and edges, panels. */
  graph?: GraphData;
  /** A grid (Phase 6): walls, weights, per-cell roles and labels, the path found. */
  grid?: GridData;
  /** What the sample returns, set by the tracer on the last step (topics whose answer is
   * not the final row). cases.json `output` comes from here, so it is the tracer's own. */
  result?: number[];
};

/** A list node; `text` replaces the value on the node (LRU shows `key:value`). */
export type ListNode = Item & { role?: Role; text?: string };
export type ListData = {
  nodes: ListNode[];
  next: Record<number, number | null>;
  /** Doubly linked lists: the back links. */
  prev?: Record<number, number | null>;
  /** Labels on nodes (head, tail, prev, curr, slow, fast), by node id. */
  labels: { label: string; id: number; tone: PointerTone }[];
  fixed?: boolean;
};
export type SlotsData = {
  layout: "stack" | "row" | "ring";
  cells: (Item | null)[];
  roles?: Record<number, Role>;
  pointers?: Pointer[];
  /** The row's title (`learn.stage.rows.<label>`). */
  label: RowLabel;
};
export type TableData = {
  label: RowLabel;
  rows: { key: number; value: number; role?: Role }[];
};

/** Graphs (Phases 4-5): nodes 0..n-1 placed once by d3-force; edges in input order. */
export type GraphNode = { id: number; role?: Role; note?: string; mark?: boolean; text?: string };
export type GraphEdge = { id: number; u: number; v: number; w: number; role?: Role; label?: string };
export type GraphData = {
  n: number;
  directed: boolean;
  weighted: boolean;
  nodes: GraphNode[];
  edges: GraphEdge[];
  /** A panel with each node's neighbour list; `hotRow` is the row being written. */
  adjacency?: number[][];
  hotRow?: number;
  /** A matrix panel (representations, Floyd-Warshall); null is drawn as ∞. */
  matrix?: { label: RowLabel; cells: (number | null)[][]; hot?: [number, number][]; via?: [number, number][] };
};

/** Grids (Phase 6): cells by index r * cols + c (0 open, 1 wall, 2-9 a weight). */
export type GridData = {
  rows: number;
  cols: number;
  cells: number[];
  start: [number, number] | null;
  goal: [number, number] | null;
  roles: Record<number, Role>;
  /** A label replacing the cell's weight (a distance, f, h). */
  text: Record<number, string>;
  /** The path found, as cell indices from start to goal. */
  path: number[];
};

export type ForestNode = {
  id: number;
  parent: number | null;
  text: string;
  role?: Role;
  /** Binary trees: which child of its parent this is. */
  side?: "L" | "R";
  /** A label on the edge from the parent (a trie's letter). */
  edge?: string;
  /** A note under the node (a distance, a count); "@key" is translated (NoteKey). */
  note?: string;
  /** A second ring (a trie node that ends a word, a root). */
  mark?: boolean;
};
export type HashData = {
  mode: "chain" | "open";
  /** Chaining: each bucket's keys; open addressing: one item or none (value −2 is a
   * tombstone). */
  buckets: Item[][];
  /** The buckets visited for the current key, in order. */
  probe: number[];
  key: number | null;
  /** "k mod m = b" for the current key. */
  home: number | null;
};

/** One frame. Snapshots, not diffs: scrubbing is O(1). */
export type Step = SceneData & {
  items: Item[];
  /** Roles by position; positions not listed are `idle`. */
  roles: Record<number, Role>;
  /** The position whose item is lifted above the row (insertion's key), or null. */
  lifted: number | null;
  pointers: Pointer[];
  /** The active range (inclusive positions), or null. */
  range: [number, number] | null;
  counters: Counters;
  narration: Narration;
  /** The `@step` id of the sample line behind this step, or null (start, end). */
  marker: string | null;
  vars: Record<string, VarValue>;
};

export type SceneKind =
  | "bars"
  | "array"
  | "growth"
  | "recursion"
  | "hanoi"
  | "bits"
  | "buckets"
  | "merge"
  | "heap"
  | "monostack"
  | "list"
  | "slots"
  | "tree"
  | "hash"
  | "bits-array"
  | "graph"
  | "grid";

/** How a topic's input is shaped. Inputs are always one flat list of integers (the DB and
 * the sample driver see only that):
 * - `values`: the values;
 * - `target`: `[target, ...values]`;
 * - `sorted-target`: `[target, ...values]` with the values ascending;
 * - `count`: `[n]` (disks, the size of a problem). */
export type InputKind = "values" | "target" | "sorted-target" | "count" | "codec";

/** A "your own" field label (`learn.input.field.<key>`). */
export type FieldKey = keyof Locale["input"]["field"];

/** How a structured input (operations, text, a graph, a grid…) is typed and stored: one
 * text per field in the UI, one flat integer list everywhere else. */
export type InputCodec = {
  fields: readonly FieldKey[];
  /** The flat input as one text per field. */
  format: (input: readonly number[]) => string[];
  /** The texts as a flat input, or a reason to refuse them. */
  parse: (texts: readonly string[]) => { values: number[] } | { error: InputError };
  /** Checks beyond the flat limits (shape, references), or null. */
  check?: (input: readonly number[]) => InputError | null;
};

export type InputSpec = {
  kind: InputKind;
  /** The `learn.input.target.<label>` name of the first number for target kinds. */
  targetLabel?: FieldKey;
  /** Picks a target for a preset (target kinds), from the values and a seeded random. */
  pickTarget?: (values: number[], next: () => number) => number;
  /** Extra checks for a typed-in target, as an error, or null when fine. */
  checkTarget?: (target: number, values: readonly number[]) => { min: number; max: number } | null;
  /** The structured input of `codec` topics. */
  codec?: InputCodec;
};

export type CounterSpec = { slot: keyof Counters; label: CounterLabel };

export type TracerDef = {
  id: TopicId;
  /** The subject folder in content/learn. */
  subject: string;
  scene: SceneKind;
  /** Topics in the same subject and group can be compared on one input: "sort" (values
   * to sort), "find" (a value to look for), "grid" (a map: the pathfinding race). */
  compareGroup?: "sort" | "find" | "grid";
  /** The array scene keeps room above the row for a lifted key (insertion-style sorts). */
  lifts?: boolean;
  /** The topic.toml `[input]` limits (Vitest checks they match). */
  limits: LearnInput;
  input: InputSpec;
  /** Named presets, each a generator from a seeded random (`learn.input.presets.<id>`).
   * Topics without them use the four sort presets. A `random` preset, when present, also
   * makes the 20 random cases of cases.json. */
  presets?: Readonly<Record<string, (next: () => number) => number[]>>;
  /** Every `@step` id the tracer emits; both samples must carry each once. */
  markers: readonly string[];
  /** Every narration key the tracer can emit (full keys under `learn.narration`). */
  narrationKeys: readonly string[];
  /** What the three counters count, in display order. */
  counters: readonly CounterSpec[];
  legend: readonly Role[];
  /** Legend names that differ from the role's default (binary search's "mid"). */
  roleLabels?: Partial<Record<Role, RoleLabel>>;
  vars: readonly string[];
  trace: (input: readonly number[]) => Step[];
};

/** The most steps a trace may hold; inputs that need more are refused up front. */
export const MAX_STEPS = 2000;

/** The usual counters of a sort: comparisons, swaps or shifts, writes. */
export const sortCounters = (moves: "swaps" | "shifts"): readonly CounterSpec[] => [
  { slot: "comparisons", label: "comparisons" },
  { slot: "moves", label: moves },
  { slot: "writes", label: "writes" },
];

/** What the samples print, from the trace's last step: its `result`, or else the final
 * items (for a sort, the sorted input). */
export function outputOf(steps: readonly Step[]): number[] {
  const last = steps[steps.length - 1]!;
  return last.result ?? last.items.map((it) => it.value);
}
