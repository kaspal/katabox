// Backtracking (ADR 0029, Phase 9): the decision tree as an indented outline, one line per
// call, in the order the search makes them. The current call is outlined, solutions are
// marked, pruned or undone branches are greyed. The panel keeps the current line in view,
// so trees of any size stay readable; screen readers get the lines as text. Above it,
// optionally, the current choice (`slots`); below it, what was found (`aux`). BoardScene
// draws the same outline under its board.
import { useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import type { DecisionData, Role } from "../engine/types";
import { Cell } from "./ArrayScene";
import { SceneFallback, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const LINE = 22;
const MARK: Partial<Record<Role, string>> = { sorted: "✓", out: "✗", key: "▶" };

export function DecisionOutline({ data, height = 240 }: { data: DecisionData; height?: number }) {
  const { t } = useTranslation();
  const box = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const el = box.current;
    if (!el || data.current == null) return;
    const y = data.current * LINE;
    if (y < el.scrollTop || y > el.scrollTop + el.clientHeight - LINE) el.scrollTop = Math.max(0, y - el.clientHeight / 2);
  }, [data.current]);
  return (
    <div>
      <div className="mb-1 text-xs font-medium text-muted-foreground">{t("learn.stage.rows.decisions")}</div>
      <div ref={box} className="overflow-y-auto rounded-md border bg-card font-mono text-xs" style={{ maxHeight: height }} data-testid="decision-outline" tabIndex={0} aria-label={t("learn.stage.rows.decisions")}>
        <ol className="py-1">
          {data.lines.map((line, k) => {
            const role = line.role ?? "idle";
            return (
              <li
                key={k}
                data-role={role}
                className="flex items-center gap-2 pr-2"
                style={{
                  height: LINE,
                  paddingLeft: 8 + line.depth * 16,
                  background: k === data.current ? `color-mix(in srgb, ${roleColor("key")} 16%, transparent)` : undefined,
                  color: role === "out" ? "var(--muted-foreground)" : undefined,
                  textDecoration: role === "out" ? "line-through" : undefined,
                }}
              >
                <span aria-hidden className="w-3 text-center" style={{ color: role === "idle" ? "var(--viz-idle)" : roleColor(role) }}>
                  {MARK[role] ?? "·"}
                </span>
                <span>{line.text}</span>
              </li>
            );
          })}
        </ol>
      </div>
    </div>
  );
}

export function DecisionScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const rows: { key: string; label: string; cells: { id: number; value: number; text?: string }[]; roles: Record<number, string> }[] = [];
  if (step.slots) rows.push({ key: "slots", label: t(`learn.stage.rows.${step.slots.label}`), cells: step.slots.cells.filter((c): c is { id: number; value: number } => c !== null), roles: step.slots.roles ?? {} });
  const cell = 34;
  const rowH = cell + 30;
  const drawRow = (row: (typeof rows)[number], y: number) => {
    const slot = Math.min(44, (width - 8) / Math.max(row.cells.length, 8));
    return (
      <g key={row.key} data-testid={`row-${row.key}`}>
        <text x={0} y={y + 10} className="fill-muted-foreground text-xs font-medium">
          {row.label}
        </text>
        {row.cells.map((c, j) => (
          <g key={`${row.key}${c.id}`}>
            <Cell item={c} role={(row.roles[j] as never) ?? "idle"} cell={Math.min(cell, slot - 6)} x={j * slot} y={y + 18} tr={tr} keyPrefix={`${row.key}-`} />
          </g>
        ))}
      </g>
    );
  };
  return (
    <div ref={ref} className="relative flex w-full flex-col gap-3" data-testid="decision-scene">
      {rows.length > 0 && (
        <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={rows.length * rowH} className="block overflow-visible">
          {rows.map((row, k) => drawRow(row, k * rowH))}
        </svg>
      )}
      {step.decision && <DecisionOutline data={step.decision} />}
      {step.aux && (
        <div data-testid="found-list" className="text-xs">
          <div className="mb-1 font-medium text-muted-foreground">{t(`learn.stage.rows.${step.aux.label}`)}</div>
          <div className="flex flex-wrap gap-1 font-mono">
            {step.aux.cells.map((c, k) => (
              <span key={c?.id ?? k} className="rounded border px-1.5 py-0.5" style={{ borderColor: roleColor("sorted") }}>
                {c?.text ?? c?.value}
              </span>
            ))}
            {step.aux.cells.length === 0 && <span className="text-muted-foreground">—</span>}
          </div>
        </div>
      )}
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
