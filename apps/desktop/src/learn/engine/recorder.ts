// A small helper tracers share: it owns the items, the counters and the sorted set, and
// records a snapshot per step.
import type { Item, Narration, Pointer, Role, SceneData, Step, VarValue } from "./types";

export type Frame = SceneData & {
  roles?: Record<number, Role>;
  lifted?: number | null;
  pointers?: Pointer[];
  range?: [number, number] | null;
  narration: Narration;
  marker: string | null;
  vars?: Record<string, VarValue>;
};

export class Recorder {
  readonly items: Item[];
  readonly steps: Step[] = [];
  comparisons = 0;
  moves = 0;
  writes = 0;
  private readonly sorted = new Set<number>();

  constructor(values: readonly number[]) {
    this.items = values.map((value, id) => ({ id, value }));
  }

  value(at: number): number {
    return this.items[at]!.value;
  }

  swap(a: number, b: number) {
    const t = this.items[a]!;
    this.items[a] = this.items[b]!;
    this.items[b] = t;
  }

  /** Mark the item now at `at` as in its final place. */
  settle(at: number) {
    this.sorted.add(this.items[at]!.id);
  }

  settleAll() {
    for (const it of this.items) this.sorted.add(it.id);
  }

  snap(f: Frame) {
    const roles: Record<number, Role> = {};
    this.items.forEach((it, p) => {
      if (this.sorted.has(it.id)) roles[p] = "sorted";
      else if (f.range && (p < f.range[0] || p > f.range[1])) roles[p] = "out";
    });
    Object.assign(roles, f.roles);
    const { roles: _r, lifted: _l, pointers: _p, range: _g, narration: _n, marker: _m, vars: _v, ...scene } = f;
    this.steps.push({
      ...scene,
      items: this.items.map((it) => ({ ...it })),
      roles,
      lifted: f.lifted ?? null,
      pointers: f.pointers ?? [],
      range: f.range ?? null,
      counters: { comparisons: this.comparisons, moves: this.moves, writes: this.writes },
      narration: f.narration,
      marker: f.marker,
      vars: f.vars ?? {},
    });
  }
}
