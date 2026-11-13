// The visualizer's drawings (ADR 0027): hand-written SVG, colours from the viz tokens, and
// every change marked twice (a dashed outline and a "•"), never by colour alone.
import { useId, type ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { cn } from "@/lib/utils";
import type { Changes } from "@/workspace/debug/visualize/diff";
import {
  CELL_H,
  MARGIN,
  NODE_R,
  layoutGraph,
  layoutTree,
  nodeWidth,
  rowCellWidth,
} from "@/workspace/debug/visualize/layout";
import {
  shownOf,
  type ArrayModel,
  type GraphModel,
  type GridModel,
  type LinkedModel,
  type MapModel,
  type Model,
  type ScalarModel,
  type SetModel,
  type TreeModel,
} from "@/workspace/debug/visualize/model";

type Props<M> = { m: M; changes: Changes; zoom: number };

const MONO = "font-mono text-[12px]";
const SUMMARY_VALUES = 30;

function clip(text: string, max: number): string {
  return text.length <= max ? text : text.slice(0, max - 1) + "…";
}

function list(values: string[]): string {
  const head = values.slice(0, SUMMARY_VALUES).join(", ");
  return values.length > SUMMARY_VALUES ? head + ", …" : head;
}

/** An SVG at its natural size times the zoom (scrolls inside its card). */
function Svg({ w, h, zoom, label, children }: { w: number; h: number; zoom: number; label: string; children: ReactNode }) {
  return (
    <svg
      role="img"
      aria-label={label}
      width={Math.ceil(w * zoom)}
      height={Math.ceil(h * zoom)}
      viewBox={`0 0 ${Math.ceil(w)} ${Math.ceil(h)}`}
      className={cn("block max-w-none", MONO)}
    >
      {children}
    </svg>
  );
}

/** The "changed" marker: a dot above the item, in the changed colour. */
function Dot({ x, y }: { x: number; y: number }) {
  return (
    <text x={x} y={y} textAnchor="middle" className="fill-viz-changed font-bold" data-changed="">
      •
    </text>
  );
}

function Box({
  x,
  y,
  w,
  text,
  changed,
  pointer,
}: {
  x: number;
  y: number;
  w: number;
  text: string;
  changed?: boolean;
  pointer?: boolean;
}) {
  return (
    <g data-changed={changed ? "" : undefined}>
      <rect
        x={x}
        y={y}
        width={w}
        height={CELL_H}
        rx={4}
        className={cn(
          "fill-viz-node",
          changed ? "stroke-viz-changed" : pointer ? "stroke-viz-pointer" : "stroke-viz-edge",
        )}
        strokeWidth={changed ? 2.5 : pointer ? 2 : 1}
        strokeDasharray={changed ? "5 3" : undefined}
      />
      <text x={x + w / 2} y={y + CELL_H / 2 + 4} textAnchor="middle" className="fill-foreground">
        {clip(text, Math.floor((w - 8) / 7.5))}
        <title>{text}</title>
      </text>
      {changed && <Dot x={x + w - 6} y={y - 3} />}
    </g>
  );
}

/** A unique, URL-safe id for an SVG marker (useId's ids hold characters url() rejects). */
function useMarkerId(): string {
  return "viz-arrow-" + useId().replace(/[^a-zA-Z0-9_-]/g, "");
}

function ArrowDefs({ id }: { id: string }) {
  return (
    <defs>
      <marker id={id} viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
        <path d="M0,0 L10,5 L0,10 z" className="fill-viz-edge" />
      </marker>
    </defs>
  );
}

// ---- Array and set ------------------------------------------------------------------------

export function ArrayView({ m, changes, zoom }: Props<ArrayModel>) {
  const { t } = useTranslation();
  const w = rowCellWidth(m.cells.map((c) => c.text));
  const top = 18;
  const atEnd = m.pointers.some((p) => p.index >= m.cells.length);
  const slots = m.cells.length + (atEnd ? 1 : 0);
  // Pointers on one index stack downwards.
  const stacks = new Map<number, string[]>();
  for (const p of m.pointers) stacks.set(p.index, [...(stacks.get(p.index) ?? []), p.name]);
  const deepest = Math.max(0, ...[...stacks.values()].map((s) => s.length));
  const width = MARGIN + Math.max(slots, 1) * w + MARGIN;
  const height = top + CELL_H + 6 + deepest * 16 + 8;
  const label = t("debug.visualize.aria.array", { name: m.name, values: list(m.cells.map((c) => c.text)) });
  return (
    <Svg w={width} h={height} zoom={zoom} label={label}>
      {m.cells.map((c, i) => (
        <g key={i}>
          <text x={MARGIN + i * w + w / 2} y={top - 6} textAnchor="middle" className="fill-muted-foreground text-[10px]">
            {i}
          </text>
          <Box x={MARGIN + i * w} y={top} w={w} text={c.text} changed={changes.cells.has(String(i))} />
        </g>
      ))}
      {[...stacks.entries()].map(([index, names]) =>
        names.map((name, k) => {
          const x = MARGIN + Math.min(index, slots - 1) * w + w / 2;
          const y = top + CELL_H + 16 + k * 16;
          const moved = changes.pointers.has(name);
          return (
            <text
              key={name}
              x={x}
              y={y}
              textAnchor="middle"
              className={cn("fill-viz-pointer", moved && "font-bold")}
              data-pointer={name}
              data-changed={moved ? "" : undefined}
            >
              ▲ {name}
              {index >= m.cells.length ? ` (${t("debug.visualize.end")})` : ""}
              {moved ? " •" : ""}
            </text>
          );
        }),
      )}
    </Svg>
  );
}

export function SetView({ m, changes, zoom }: Props<SetModel>) {
  const { t } = useTranslation();
  const w = rowCellWidth(m.cells.map((c) => c.text));
  const width = MARGIN * 2 + Math.max(m.cells.length, 1) * (w + 6);
  const label = t("debug.visualize.aria.set", { name: m.name, values: list(m.cells.map((c) => c.text)) });
  return (
    <Svg w={width} h={CELL_H + 20} zoom={zoom} label={label}>
      {m.cells.map((c, i) => (
        <Box key={i} x={MARGIN + i * (w + 6)} y={10} w={w} text={c.text} changed={changes.cells.has(String(i))} />
      ))}
    </Svg>
  );
}

// ---- Grid ---------------------------------------------------------------------------------

export function GridView({ m, changes, zoom }: Props<GridModel>) {
  const { t } = useTranslation();
  const w = rowCellWidth(m.rows.flat().map((c) => c.text));
  const left = MARGIN + 12;
  const top = 18;
  const pointerAt = new Map(m.pointers.map((p) => [`${p.r},${p.c}`, p.name]));
  const width = left + m.cols * w + MARGIN;
  const height = top + m.rows.length * CELL_H + 8;
  const label = t("debug.visualize.aria.grid", { name: m.name, rows: m.len, cols: m.cols });
  return (
    <Svg w={width} h={height} zoom={zoom} label={label}>
      {Array.from({ length: m.cols }, (_, c) => (
        <text key={c} x={left + c * w + w / 2} y={top - 6} textAnchor="middle" className="fill-muted-foreground text-[10px]">
          {c}
        </text>
      ))}
      {m.rows.map((row, r) => (
        <g key={r}>
          <text x={left - 8} y={top + r * CELL_H + CELL_H / 2 + 4} textAnchor="end" className="fill-muted-foreground text-[10px]">
            {r}
          </text>
          {row.map((cell, c) => {
            const p = pointerAt.get(`${r},${c}`);
            return (
              <g key={c}>
                <Box x={left + c * w} y={top + r * CELL_H} w={w} text={cell.text} changed={changes.cells.has(`${r},${c}`)} pointer={p !== undefined} />
                {p !== undefined && (
                  <text x={left + c * w + 3} y={top + r * CELL_H + 10} className="fill-viz-pointer text-[9px]" data-pointer={p}>
                    {p}
                    {changes.pointers.has(p) ? " •" : ""}
                  </text>
                )}
              </g>
            );
          })}
        </g>
      ))}
    </Svg>
  );
}

// ---- Linked list --------------------------------------------------------------------------

export function LinkedListView({ m, changes, zoom }: Props<LinkedModel>) {
  const { t } = useTranslation();
  const arrow = useMarkerId();
  const w = rowCellWidth(m.nodes.map((n) => n.text));
  const gap = 34;
  const top = 18;
  const step = w + gap;
  const deepest = Math.max(0, ...m.nodes.map((n) => n.labels.length));
  const tail = m.cycleTo === null ? (m.more ? "…" : "null") : "";
  const width = MARGIN + m.nodes.length * step + (tail ? 40 : 0) + MARGIN;
  // Room below the boxes for the cycle's curve (its lowest point is 3/4 of this) and,
  // under it, its label.
  const loop = m.cycleTo !== null ? 46 : 0;
  const height = top + CELL_H + 8 + loop + deepest * 16 + 10;
  const values = m.nodes.map((n) => n.text);
  const label = t("debug.visualize.aria.linked", {
    name: m.name,
    values: list(values) + (m.cycleTo !== null ? ` ↺ ${values[m.cycleTo]}` : ""),
  });
  const cx = (i: number) => MARGIN + i * step + w / 2;
  return (
    <Svg w={width} h={height} zoom={zoom} label={label}>
      <ArrowDefs id={arrow} />
      {m.nodes.map((n, i) => {
        const x = MARGIN + i * step;
        const last = i === m.nodes.length - 1;
        return (
          <g key={n.key} data-node={n.key}>
            <Box x={x} y={top} w={w} text={n.text} changed={changes.nodes.has(n.key)} />
            {(!last || tail) && (
              <line
                x1={x + w}
                y1={top + CELL_H / 2}
                x2={x + w + gap - 2}
                y2={top + CELL_H / 2}
                className="stroke-viz-edge"
                markerEnd={`url(#${arrow})`}
              />
            )}
            {n.labels.map((l, k) => (
              <text
                key={l}
                x={x + w / 2}
                y={top + CELL_H + loop + 16 + k * 16}
                textAnchor="middle"
                className={cn("fill-viz-pointer", changes.pointers.has(l) && "font-bold")}
                data-pointer={l}
              >
                ▲ {l}
                {changes.pointers.has(l) ? " •" : ""}
              </text>
            ))}
          </g>
        );
      })}
      {tail && (
        <text x={MARGIN + m.nodes.length * step + 4} y={top + CELL_H / 2 + 4} className="fill-muted-foreground">
          {tail}
        </text>
      )}
      {m.cycleTo !== null && m.nodes.length > 0 && (
        <g data-cycle="">
          <path
            d={`M ${cx(m.nodes.length - 1)} ${top + CELL_H} C ${cx(m.nodes.length - 1)} ${top + CELL_H + loop}, ${cx(m.cycleTo)} ${top + CELL_H + loop}, ${cx(m.cycleTo)} ${top + CELL_H + 2}`}
            fill="none"
            className="stroke-viz-edge"
            strokeDasharray="3 2"
            markerEnd={`url(#${arrow})`}
          />
          <text
            x={(cx(m.nodes.length - 1) + cx(m.cycleTo)) / 2}
            y={top + CELL_H + loop - 1}
            textAnchor="middle"
            className="fill-muted-foreground text-[10px]"
          >
            ↺ {t("debug.visualize.cycle", { value: m.nodes[m.cycleTo].text })}
          </text>
        </g>
      )}
    </Svg>
  );
}

// ---- Tree and heap ------------------------------------------------------------------------

export function TreeView({ m, changes, zoom }: Props<TreeModel>) {
  const { t } = useTranslation();
  const { points, width, height } = layoutTree(m);
  const deepest = Math.max(0, ...m.nodes.map((n) => n.labels.length));
  const label = t(m.heap ? "debug.visualize.aria.heap" : "debug.visualize.aria.tree", { name: m.name, count: m.nodes.length });
  return (
    <Svg w={width} h={height + deepest * 14} zoom={zoom} label={label}>
      {m.nodes.map((n, i) =>
        (["left", "right"] as const).map((side) => {
          const c = n[side];
          if (c === null) return null;
          return (
            <line
              key={`${i}-${side}`}
              x1={points[i].x}
              y1={points[i].y + NODE_R}
              x2={points[c].x}
              y2={points[c].y - NODE_R}
              className="stroke-viz-edge"
            />
          );
        }),
      )}
      {m.nodes.map((n, i) => {
        const changed = changes.nodes.has(n.key);
        const { x, y } = points[i];
        return (
          <g key={n.key} data-node={n.key} data-changed={changed ? "" : undefined}>
            <rect
              x={x - nodeWidth(n.text) / 2}
              y={y - NODE_R}
              width={nodeWidth(n.text)}
              height={NODE_R * 2}
              rx={NODE_R}
              className={cn("fill-viz-node", changed ? "stroke-viz-changed" : "stroke-viz-edge")}
              strokeWidth={changed ? 2.5 : 1}
              strokeDasharray={changed ? "5 3" : undefined}
            />
            <text x={x} y={y + 4} textAnchor="middle" className="fill-foreground">
              {clip(n.text, Math.max(4, Math.floor((nodeWidth(n.text) - 8) / 7.5)))}
              <title>{n.text}</title>
            </text>
            {m.heap && (
              <text x={x} y={y - NODE_R - 3} textAnchor="middle" className="fill-muted-foreground text-[9px]">
                {i}
              </text>
            )}
            {changed && <Dot x={x + nodeWidth(n.text) / 2} y={y - NODE_R + 2} />}
            {n.labels.map((l, k) => (
              <text
                key={l}
                x={x}
                y={y + NODE_R + 12 + k * 13}
                textAnchor="middle"
                className={cn("fill-viz-pointer text-[10px]", changes.pointers.has(l) && "font-bold")}
                data-pointer={l}
              >
                {l}
                {changes.pointers.has(l) ? " •" : ""}
              </text>
            ))}
          </g>
        );
      })}
    </Svg>
  );
}

// ---- Graph --------------------------------------------------------------------------------

export function GraphView({ m, changes, zoom }: Props<GraphModel>) {
  const { t } = useTranslation();
  const arrow = useMarkerId();
  const { points, size } = layoutGraph(m);
  const label = t("debug.visualize.aria.graph", { name: m.name, nodes: m.total, edges: m.edges.length });
  const ref = `url(#${arrow})`;
  return (
    <Svg w={size} h={size} zoom={zoom} label={label}>
      <ArrowDefs id={arrow} />
      {m.edges.map((e) => {
        const key = `${m.nodes[e.a]}-${m.nodes[e.b]}`;
        const changed = changes.edges.has(key);
        const cls = cn(changed ? "stroke-viz-changed" : "stroke-viz-edge");
        const width = changed ? 2.5 : 1.2;
        const dash = changed ? "5 3" : undefined;
        const p = points[e.a];
        if (e.a === e.b) {
          // A self-loop: a small circle above the node.
          return (
            <g key={key} data-edge={key}>
              <circle cx={p.x} cy={p.y - NODE_R - 8} r={9} fill="none" className={cls} strokeWidth={width} strokeDasharray={dash} />
              {e.count > 1 && (
                <text x={p.x + 12} y={p.y - NODE_R - 14} className="fill-muted-foreground text-[10px]">
                  ×{e.count}
                </text>
              )}
            </g>
          );
        }
        const q = points[e.b];
        const len = Math.hypot(q.x - p.x, q.y - p.y) || 1;
        const ux = (q.x - p.x) / len;
        const uy = (q.y - p.y) / len;
        return (
          <g key={key} data-edge={key}>
            <line
              x1={p.x + ux * NODE_R}
              y1={p.y + uy * NODE_R}
              x2={q.x - ux * (NODE_R + 2)}
              y2={q.y - uy * (NODE_R + 2)}
              className={cls}
              strokeWidth={width}
              strokeDasharray={dash}
              markerEnd={ref}
              markerStart={e.both ? ref : undefined}
            />
            {e.count > 1 && (
              <text x={(p.x + q.x) / 2 + 4} y={(p.y + q.y) / 2 - 4} className="fill-muted-foreground text-[10px]">
                ×{e.count}
              </text>
            )}
          </g>
        );
      })}
      {m.nodes.map((n, i) => (
        <g key={i} data-node={n}>
          <circle cx={points[i].x} cy={points[i].y} r={NODE_R} className="fill-viz-node stroke-viz-edge" />
          <text x={points[i].x} y={points[i].y + 4} textAnchor="middle" className="fill-foreground">
            {clip(n, 4)}
            <title>{n}</title>
          </text>
        </g>
      ))}
    </Svg>
  );
}

// ---- Map and scalar (HTML) ----------------------------------------------------------------

export function MapView({ m, changes }: Props<MapModel>) {
  const { t } = useTranslation();
  return (
    <div className="space-y-1">
      <table className={cn("border-collapse", MONO)} aria-label={t("debug.visualize.aria.map", { name: m.name, count: m.len })}>
        <thead>
          <tr className="text-left text-muted-foreground">
            <th scope="col" className="w-4" />
            <th scope="col" className="pr-4 font-normal">{t("debug.visualize.key")}</th>
            <th scope="col" className="font-normal">{t("debug.visualize.value")}</th>
          </tr>
        </thead>
        <tbody>
          {m.entries.map((e) => {
            const changed = changes.cells.has(e.key);
            return (
              <tr key={e.key} data-changed={changed ? "" : undefined}>
                <td className="text-viz-changed font-bold" aria-label={changed ? t("debug.visualize.changed") : undefined}>
                  {changed ? "•" : ""}
                </td>
                <td className={cn("border-y border-l px-2 py-0.5", changed && "border-2 border-dashed border-viz-changed")}>{e.key}</td>
                <td className={cn("border px-2 py-0.5", changed && "border-2 border-dashed border-viz-changed")}>{e.value}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
      {m.entries.length === 0 && <p className="text-xs text-muted-foreground">{t("debug.visualize.empty_value")}</p>}
      {changes.removed.length > 0 && (
        <p className="text-xs text-muted-foreground">{t("debug.visualize.removed", { keys: changes.removed.join(", ") })}</p>
      )}
    </div>
  );
}

export function ScalarView({ m, changes }: Props<ScalarModel>) {
  const changed = changes.cells.size > 0;
  return (
    <p className={cn(MONO)} data-changed={changed ? "" : undefined}>
      {m.text}
      {changed && <span className="ml-1 font-bold text-viz-changed"> •</span>}
    </p>
  );
}

// ---- Any model ----------------------------------------------------------------------------

export function ModelView({ m, changes, zoom }: Props<Model>) {
  switch (m.kind) {
    case "array":
      return <ArrayView m={m} changes={changes} zoom={zoom} />;
    case "grid":
      return <GridView m={m} changes={changes} zoom={zoom} />;
    case "linked":
      return <LinkedListView m={m} changes={changes} zoom={zoom} />;
    case "tree":
      return <TreeView m={m} changes={changes} zoom={zoom} />;
    case "graph":
      return <GraphView m={m} changes={changes} zoom={zoom} />;
    case "map":
      return <MapView m={m} changes={changes} zoom={zoom} />;
    case "set":
      return <SetView m={m} changes={changes} zoom={zoom} />;
    case "scalar":
      return <ScalarView m={m} changes={changes} zoom={zoom} />;
  }
}

/** A card for one variable: its name, what is shown of it, how many changes, the drawing. */
export function ModelCard({ m, changes, zoom, large }: Props<Model> & { large?: boolean }) {
  const { t } = useTranslation();
  const titleId = useId();
  const shown = shownOf(m);
  const count = changes.cells.size + changes.nodes.size + changes.pointers.size + changes.edges.size + changes.removed.length;
  return (
    <figure
      tabIndex={0}
      aria-labelledby={titleId}
      className="min-w-0 space-y-1 rounded-md border bg-card p-2 focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
      data-testid={`viz-${m.name}`}
      data-kind={m.kind}
    >
      <figcaption id={titleId} className="flex flex-wrap items-baseline gap-x-2 text-xs">
        <span className="font-mono font-semibold">{m.name}</span>
        {shown && (
          <span className="text-muted-foreground">
            {shown.total === null
              ? t("debug.visualize.showingSome", { shown: shown.shown })
              : t("debug.visualize.showing", { shown: shown.shown, total: shown.total })}
          </span>
        )}
        {count > 0 && (
          <span className="text-viz-changed">
            • {t("debug.visualize.changedCount", { count })}
          </span>
        )}
      </figcaption>
      <div className={cn("overflow-auto", large ? "max-h-[60vh]" : "max-h-72")}>
        <ModelView m={m} changes={changes} zoom={zoom} />
      </div>
    </figure>
  );
}

/** The legend for the change marker (dashed outline + dot). */
export function ChangeLegend() {
  const { t } = useTranslation();
  return (
    <p className="flex items-center gap-2 text-xs text-muted-foreground">
      <svg width="28" height="16" aria-hidden className="shrink-0">
        <rect x="2" y="2" width="24" height="12" rx="3" className="fill-viz-node stroke-viz-changed" strokeWidth={2} strokeDasharray="5 3" />
      </svg>
      <span>
        <span className="font-bold text-viz-changed">•</span> {t("debug.visualize.changed")}
      </span>
    </p>
  );
}
