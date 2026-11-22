// Visual custom-test builders (expansion M9, ADR 0036): trees, rows, grids and graphs for a
// custom test's parameters. Each builder reads the parameter's JSON text and writes JSON
// text back, so the text field stays editable; when the text doesn't parse, the builder
// asks to fix it as JSON first. Every action has a keyboard path (focusable nodes and
// cells, Enter, Delete, arrow keys).
import { useMemo, useState, type KeyboardEvent } from "react";
import { useTranslation } from "react-i18next";
import { ArrowDown, ArrowUp, Plus, Trash2, Wrench } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { layoutGraph } from "@/learn/engine/graphLayout";
import { cn } from "@/lib/utils";
import { elementType } from "@/workspace/results/validate";
import {
  addChild,
  buildersFor,
  decodeTree,
  emptyTree,
  encodeTree,
  gridCharset,
  nodeCount,
  placeTree,
  removeNode,
  setCell,
  type BuilderKind,
} from "./model";

type Props = { name: string; text: string; onText: (text: string) => void };

const parse = (text: string): { ok: true; value: unknown } | { ok: false } => {
  try {
    return { ok: true, value: JSON.parse(text.trim() === "" ? "null" : text) };
  } catch {
    return { ok: false };
  }
};

// --- Rows: arrays of values, one row each ------------------------------------------------

function RowsEditor({ name, type, value, onValue }: { name: string; type: string; value: unknown[]; onValue: (v: unknown[]) => void }) {
  const { t } = useTranslation();
  const inner = type === "ListNode" ? "int" : elementType(type);
  const isString = inner === "string";
  const show = (v: unknown) => (isString && typeof v === "string" ? v : JSON.stringify(v));
  const read = (s: string): unknown => {
    if (isString) return s;
    try {
      return JSON.parse(s);
    } catch {
      return s;
    }
  };
  const blank = (): unknown => (isString ? "" : inner === "bool" ? false : inner.endsWith("[]") ? [] : 0);
  const move = (i: number, d: number) => {
    const next = [...value];
    [next[i], next[i + d]] = [next[i + d], next[i]];
    onValue(next);
  };
  return (
    <div className="space-y-1.5" data-testid={`builder-rows-${name}`}>
      <ol className="space-y-1">
        {value.map((v, i) => (
          <li key={i} className="flex items-center gap-1">
            <span className="w-6 text-right font-mono text-[11px] text-muted-foreground">{i}</span>
            <Input
              value={show(v)}
              onChange={(e) => onValue(value.map((x, k) => (k === i ? read(e.target.value) : x)))}
              aria-label={t("builders.row", { name, index: i })}
              spellCheck={false}
              className="h-7 font-mono text-xs"
            />
            <Button size="icon" variant="ghost" className="size-7" disabled={i === 0} onClick={() => move(i, -1)} aria-label={t("builders.up", { index: i })}>
              <ArrowUp aria-hidden />
            </Button>
            <Button size="icon" variant="ghost" className="size-7" disabled={i === value.length - 1} onClick={() => move(i, 1)} aria-label={t("builders.down", { index: i })}>
              <ArrowDown aria-hidden />
            </Button>
            <Button size="icon" variant="ghost" className="size-7" onClick={() => onValue(value.filter((_, k) => k !== i))} aria-label={t("builders.removeRow", { index: i })}>
              <Trash2 aria-hidden />
            </Button>
          </li>
        ))}
      </ol>
      <Button size="sm" variant="outline" className="h-7" onClick={() => onValue([...value, blank()])}>
        <Plus aria-hidden />
        {t("builders.addRow")}
      </Button>
    </div>
  );
}

// --- Tree: click a node to edit it, add children or remove it ----------------------------

function TreeBuilder({ name, value, onValue }: { name: string; value: unknown; onValue: (v: unknown) => void }) {
  const { t } = useTranslation();
  const tree = decodeTree(value) ?? emptyTree();
  const placed = placeTree(tree);
  const [selected, setSelected] = useState<number | null>(tree.root);
  const sel = selected !== null && placed.some((p) => p.id === selected) ? selected : null;
  const cols = Math.max(1, placed.length);
  const depth = Math.max(0, ...placed.map((p) => p.depth)) + 1;
  const at = new Map(placed.map((p) => [p.id, p]));
  const X = (x: number) => ((x + 0.5) / cols) * 100;
  const Y = (d: number) => ((d + 0.5) / depth) * 100;
  const commit = (next: ReturnType<typeof emptyTree>, select: number | null) => {
    onValue(encodeTree(next));
    setSelected(select);
  };
  const onKey = (e: KeyboardEvent, id: number) => {
    if (e.key === "Delete" || e.key === "Backspace") {
      e.preventDefault();
      const parent = at.get(id)?.parent ?? null;
      commit(removeNode(tree, id), parent);
    }
  };
  return (
    <div className="space-y-2" data-testid={`builder-tree-${name}`}>
      {tree.root === null ? (
        <Button size="sm" variant="outline" className="h-7" onClick={() => commit(addChild(tree, null, "left", 0), 0)}>
          <Plus aria-hidden />
          {t("builders.addRoot")}
        </Button>
      ) : (
        <div className="relative w-full rounded-md border bg-muted/30" style={{ height: `${depth * 44 + 8}px` }}>
          <svg aria-hidden className="absolute inset-0 size-full" preserveAspectRatio="none" viewBox="0 0 100 100">
            {placed
              .filter((p) => p.parent !== null)
              .map((p) => {
                const q = at.get(p.parent!)!;
                return (
                  <line
                    key={p.id}
                    x1={X(q.x)}
                    y1={Y(q.depth)}
                    x2={X(p.x)}
                    y2={Y(p.depth)}
                    stroke="var(--border)"
                    strokeWidth={1.5}
                    vectorEffect="non-scaling-stroke"
                  />
                );
              })}
          </svg>
          {placed.map((p) => (
            <button
              key={p.id}
              type="button"
              onClick={() => setSelected(p.id)}
              onKeyDown={(e) => onKey(e, p.id)}
              aria-pressed={sel === p.id}
              aria-label={t("builders.node", { value: tree.val[p.id] })}
              data-node={tree.val[p.id]}
              className={cn(
                "absolute flex size-8 -translate-x-1/2 -translate-y-1/2 items-center justify-center rounded-full border bg-card font-mono text-xs",
                sel === p.id && "border-primary ring-2 ring-primary/40",
              )}
              style={{ left: `${X(p.x)}%`, top: `${Y(p.depth)}%` }}
            >
              {tree.val[p.id]}
            </button>
          ))}
        </div>
      )}
      {sel !== null && (
        <div className="flex flex-wrap items-center gap-1.5" data-testid="tree-selected">
          <Input
            type="number"
            value={tree.val[sel]}
            onChange={(e) => {
              const v = e.target.value === "" ? 0 : Number.parseInt(e.target.value, 10);
              if (Number.isSafeInteger(v)) commit({ ...tree, val: tree.val.map((x, k) => (k === sel ? v : x)) }, sel);
            }}
            aria-label={t("builders.nodeValue")}
            className="h-7 w-24 font-mono text-xs"
          />
          {(["left", "right"] as const).map((side) => (
            <Button
              key={side}
              size="sm"
              variant="outline"
              className="h-7"
              disabled={tree[side][sel] !== null}
              onClick={() => {
                const next = addChild(tree, sel, side, 0);
                commit(next, next.val.length - 1);
              }}
            >
              <Plus aria-hidden />
              {t(side === "left" ? "builders.addLeft" : "builders.addRight")}
            </Button>
          ))}
          <Button size="sm" variant="ghost" className="h-7" onClick={() => commit(removeNode(tree, sel), at.get(sel)?.parent ?? null)}>
            <Trash2 aria-hidden />
            {t("builders.removeNode")}
          </Button>
        </div>
      )}
    </div>
  );
}

// --- Grid: click (or Enter on) a cell to cycle its character -----------------------------

function GridBuilder({ name, value, onValue }: { name: string; value: string[]; onValue: (v: string[]) => void }) {
  const { t } = useTranslation();
  const charset = useMemo(() => gridCharset(value), [value]);
  const cycle = (r: number, c: number) => {
    const ch = value[r]![c]!;
    const next = charset[(charset.indexOf(ch) + 1) % charset.length]!;
    onValue(setCell(value, r, c, next));
  };
  const onKey = (e: KeyboardEvent<HTMLButtonElement>, r: number, c: number) => {
    const d: Record<string, [number, number]> = { ArrowUp: [-1, 0], ArrowDown: [1, 0], ArrowLeft: [0, -1], ArrowRight: [0, 1] };
    const step = d[e.key];
    if (!step) return;
    e.preventDefault();
    const target = e.currentTarget.closest("table")?.querySelector<HTMLButtonElement>(`[data-cell="${r + step[0]}-${c + step[1]}"]`);
    target?.focus();
  };
  if (value.length === 0) return <p className="text-xs text-muted-foreground">{t("builders.gridEmpty")}</p>;
  return (
    <div className="space-y-1" data-testid={`builder-grid-${name}`}>
      <table className="border-collapse" aria-label={t("builders.grid", { name })}>
        <tbody>
          {value.map((row, r) => (
            <tr key={r}>
              {[...row].map((ch, c) => (
                <td key={c} className="p-0">
                  <button
                    type="button"
                    data-cell={`${r}-${c}`}
                    onClick={() => cycle(r, c)}
                    onKeyDown={(e) => onKey(e, r, c)}
                    aria-label={t("builders.cell", { row: r, col: c, value: ch })}
                    className={cn("size-7 border font-mono text-xs", ch === "#" ? "bg-foreground/80 text-background" : "bg-card")}
                  >
                    {ch}
                  </button>
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
      <p className="text-[11px] text-muted-foreground">{t("builders.gridHint", { chars: charset.join(" ") })}</p>
    </div>
  );
}

// --- Graph: click one node, then another, to draw an edge --------------------------------

function GraphBuilder({ name, value, onValue }: { name: string; value: number[][]; onValue: (v: number[][]) => void }) {
  const { t } = useTranslation();
  const [extra, setExtra] = useState(0);
  const [from, setFrom] = useState<number | null>(null);
  const weighted = value.some((e) => e.length === 3);
  const n = Math.max(nodeCount(value), 1) + extra;
  const layout = layoutGraph(n, value.map((e) => ({ u: e[0]!, v: e[1]! })));
  const height = Math.round(Math.min(260, Math.max(140, 180 * layout.aspect)));
  const pos = (k: number) => layout.pos[k] ?? { x: 0.5, y: 0.5 };
  const pick = (k: number) => {
    if (from === null) setFrom(k);
    else if (from === k) setFrom(null);
    else {
      onValue([...value, weighted ? [from, k, 1] : [from, k]]);
      setFrom(null);
    }
  };
  const P = (v: number) => 6 + v * 88;
  return (
    <div className="space-y-2" data-testid={`builder-graph-${name}`}>
      <div className="relative w-full rounded-md border bg-muted/30" style={{ height }}>
        <svg aria-hidden className="absolute inset-0 size-full" preserveAspectRatio="none" viewBox="0 0 100 100">
          {value.map((e, i) => (
            <line
              key={i}
              x1={P(pos(e[0]!).x)}
              y1={P(pos(e[0]!).y)}
              x2={P(pos(e[1]!).x)}
              y2={P(pos(e[1]!).y)}
              stroke="var(--border)"
              strokeWidth={1.5}
              vectorEffect="non-scaling-stroke"
            />
          ))}
        </svg>
        {Array.from({ length: n }, (_, k) => (
          <button
            key={k}
            type="button"
            onClick={() => pick(k)}
            aria-pressed={from === k}
            aria-label={t(from === null ? "builders.graphNode" : "builders.graphNodeTo", { node: k, from })}
            data-graph-node={k}
            className={cn(
              "absolute flex size-8 -translate-x-1/2 -translate-y-1/2 items-center justify-center rounded-full border bg-card font-mono text-xs",
              from === k && "border-primary ring-2 ring-primary/40",
            )}
            style={{ left: `${P(pos(k).x)}%`, top: `${P(pos(k).y)}%` }}
          >
            {k}
          </button>
        ))}
      </div>
      <p className="text-[11px] text-muted-foreground">{t("builders.graphHint")}</p>
      <ul className="flex flex-wrap gap-1.5" aria-label={t("builders.edges")}>
        {value.map((e, i) => (
          <li key={i} className="flex items-center gap-1 rounded-md border px-1.5 py-0.5 font-mono text-xs">
            {e[0]}–{e[1]}
            {weighted && (
              <Input
                type="number"
                value={e[2] ?? 1}
                onChange={(ev) => {
                  // An emptied field counts as 0, so typing a new number replaces it.
                  const w = ev.target.value === "" ? 0 : Number.parseInt(ev.target.value, 10);
                  if (Number.isSafeInteger(w)) onValue(value.map((x, k) => (k === i ? [x[0]!, x[1]!, w] : x)));
                }}
                aria-label={t("builders.weight", { u: e[0], v: e[1] })}
                className="h-6 w-16 font-mono text-xs"
              />
            )}
            <Button size="icon" variant="ghost" className="size-6" onClick={() => onValue(value.filter((_, k) => k !== i))} aria-label={t("builders.removeEdge", { u: e[0], v: e[1] })}>
              <Trash2 aria-hidden />
            </Button>
          </li>
        ))}
      </ul>
      <Button size="sm" variant="outline" className="h-7" onClick={() => setExtra((x) => x + 1)}>
        <Plus aria-hidden />
        {t("builders.addNode")}
      </Button>
    </div>
  );
}

// --- The switch under a custom test's parameter ------------------------------------------

/** "Build" for one parameter: the right builder, a "View as" choice, JSON kept in sync. */
export function ParamBuilder({ name, type, text, onText }: Props & { type: string }) {
  const { t } = useTranslation();
  const parsed = parse(text);
  const kinds = buildersFor(type, name, parsed.ok ? parsed.value : null);
  const [open, setOpen] = useState(false);
  const [kind, setKind] = useState<BuilderKind | null>(null);
  if (kinds.length === 0) return null;
  const active = kind && kinds.includes(kind) ? kind : kinds[0]!;
  const write = (v: unknown) => onText(JSON.stringify(v));
  const value = parsed.ok ? parsed.value : null;
  const arrayValue = Array.isArray(value) ? value : value === null ? [] : null;
  return (
    <div className="mt-1 space-y-2">
      <Button
        size="sm"
        variant="ghost"
        className="h-6 px-1.5 text-xs"
        aria-expanded={open}
        onClick={() => setOpen((o) => !o)}
        data-testid={`build-${name}`}
      >
        <Wrench aria-hidden />
        {t(open ? "builders.hide" : "builders.show")}
      </Button>
      {open && (
        <div className="space-y-2 rounded-md border p-2">
          {kinds.length > 1 && (
            <ToggleGroup
              type="single"
              variant="outline"
              size="sm"
              aria-label={t("builders.viewAs")}
              value={active}
              onValueChange={(v) => v && setKind(v as BuilderKind)}
            >
              {kinds.map((k) => (
                <ToggleGroupItem key={k} value={k} className="h-6 text-xs">
                  {t(`builders.kind.${k}`)}
                </ToggleGroupItem>
              ))}
            </ToggleGroup>
          )}
          {!parsed.ok || (active !== "tree" && arrayValue === null) ? (
            <p role="status" className="text-xs text-muted-foreground" data-testid="builder-json">
              {t("builders.fixJson")}
            </p>
          ) : active === "tree" ? (
            <TreeBuilder name={name} value={value ?? []} onValue={write} />
          ) : active === "grid" ? (
            arrayValue!.every((r) => typeof r === "string") ? (
              <GridBuilder name={name} value={arrayValue as string[]} onValue={write} />
            ) : (
              <p className="text-xs text-muted-foreground">{t("builders.fixJson")}</p>
            )
          ) : active === "graph" ? (
            arrayValue!.every((r) => Array.isArray(r) && r.length >= 2 && r.every((x) => Number.isSafeInteger(x) && x >= 0)) ? (
              <GraphBuilder name={name} value={arrayValue as number[][]} onValue={write} />
            ) : (
              <p className="text-xs text-muted-foreground">{t("builders.fixJson")}</p>
            )
          ) : (
            <RowsEditor name={name} type={type} value={arrayValue!} onValue={write} />
          )}
        </div>
      )}
    </div>
  );
}
