// Stacks, queues, deques and ring buffers (ADR 0029, Phase 2). `slots.layout`:
// - "stack": a column, bottom first; the top is labelled;
// - "row": a queue or deque, front on the left, with front / back pointers;
// - "ring": a fixed array of `capacity` slots, empty ones dashed, head / tail pointers,
//   and an arrow from the last slot back to the first (the wrap).
// Values keep their identity, so a push drops in and a pop lifts out. The aux row (the
// values popped or returned so far) sits underneath.
import { useTranslation } from "react-i18next";
import { Cell } from "./ArrayScene";
import { Pointers, SceneFallback, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const CELL = 46;

export function SlotsScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const slots = step.slots ?? { layout: "row" as const, cells: [], label: "storage" as const };
  const cells = slots.cells;
  const stack = slots.layout === "stack";
  const n = Math.max(cells.length, stack ? 6 : 1);
  const slot = stack ? CELL + 6 : Math.min(64, width / Math.max(n, 6));
  const cell = Math.min(CELL, slot - 8);
  const auxCells = step.aux?.cells ?? [];
  const auxSlot = Math.min(56, width / Math.max(auxCells.length, 8));
  const auxCell = Math.min(40, auxSlot - 6);

  // Stack: a column on the left, bottom at `base`; rows and rings: one row at the top.
  const base = stack ? 22 + n * slot : 0;
  const rowY = 28;
  const pointerY = rowY + cell + 8;
  const auxY = stack ? 34 : slots.layout === "ring" ? pointerY + 84 : pointerY + 44;
  const auxX = stack ? cell + 70 : 0;
  const height = stack ? base + 12 : auxY + auxCell + 24;
  const cx = (k: number) => (stack ? 8 : k * slot + (slot - cell) / 2);
  const cy = (k: number) => (stack ? base - (k + 1) * slot : rowY);

  return (
    <div ref={ref} className="relative w-full" data-testid="slots-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        <text x={stack ? 8 : 0} y={14} className="fill-muted-foreground text-xs font-semibold uppercase tracking-wide">
          {t(`learn.stage.rows.${slots.label}`)}
        </text>
        {Array.from({ length: n }, (_, k) => (
          <rect
            key={`s${k}`}
            x={cx(k)}
            y={cy(k)}
            width={cell}
            height={cell}
            rx={8}
            fill="none"
            stroke="var(--border)"
            strokeDasharray={cells[k] ? undefined : "5 4"}
            data-slot={k}
            data-empty={cells[k] ? undefined : true}
          />
        ))}
        {stack && <line x1={2} x2={cell + 14} y1={base + 3} y2={base + 3} stroke="var(--viz-edge)" strokeWidth={2.5} />}
        {slots.layout === "ring" && n > 1 && (
          <g aria-hidden style={{ color: "var(--viz-edge)" }}>
            <path
              d={`M${cx(n - 1) + cell / 2} ${pointerY + 30} C${cx(n - 1) + cell / 2} ${pointerY + 48}, ${cx(0) + cell / 2} ${pointerY + 48}, ${cx(0) + cell / 2} ${pointerY + 32}`}
              fill="none"
              stroke="currentColor"
              strokeWidth={1.6}
              strokeDasharray="4 3"
            />
            <path d={`M${cx(0) + cell / 2} ${pointerY + 28} l-4.5 7 h9 z`} fill="currentColor" />
            <text x={(cx(0) + cx(n - 1) + cell) / 2} y={pointerY + 46} textAnchor="middle" className="fill-muted-foreground text-[10px]">
              {t("learn.stage.wraps")}
            </text>
          </g>
        )}
        {cells.map((c, k) =>
          c ? (
            <g key={c.id} data-pos={k}>
              <Cell item={c} role={slots.roles?.[k] ?? "idle"} cell={cell} x={cx(k)} y={cy(k)} tr={tr} />
            </g>
          ) : null,
        )}
        {stack
          ? (slots.pointers ?? []).map((p) => (
              <text
                key={p.label}
                x={cx(0) + cell + 8}
                y={cy(p.at) + cell / 2 + 4}
                className="font-mono text-xs font-semibold"
                style={{ fill: `var(--viz-ptr-${p.tone})` }}
                data-pointer={p.label}
              >
                ← {p.label}
              </text>
            ))
          : <Pointers pointers={slots.pointers ?? []} slot={slot} y={pointerY} transition={tr.move} />}
        {step.aux && (
          <g data-testid="aux-row">
            <text x={auxX} y={auxY - 8} className="fill-muted-foreground text-xs font-medium">
              {t(`learn.stage.rows.${step.aux.label}`)}
            </text>
            {auxCells.map((c, k) =>
              c ? (
                <g key={`a${c.id}`}>
                  <Cell item={c} role={step.aux!.roles?.[k] ?? "idle"} cell={auxCell} x={auxX + k * auxSlot} y={auxY} tr={tr} keyPrefix="aux-" />
                </g>
              ) : null,
            )}
          </g>
        )}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
