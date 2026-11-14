// Strings (ADR 0029, Phase 9): KMP, Z, Rabin-Karp, Manacher, suffix arrays. Letter cells
// with their index above; a pattern row shifted under the text at `shift`; number rows
// (failure, Z, radius) aligned with the string they describe; and a list panel (sorted
// suffixes). Matching letters, the window and the current cell are told by role and
// outline, and every row has a label.
import { useTranslation } from "react-i18next";
import type { Role, StringRow } from "../engine/types";
import { SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

export function StringScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const s = step.strings;
  const rows = s?.rows ?? [];
  const widest = Math.max(1, ...rows.map((r) => r.offset + r.cells.length));
  const label = 92;
  const size = Math.max(18, Math.min(34, Math.floor((width - label - 4) / widest)));
  const rowH = size + 10;
  const top = 16;
  const lines = s?.lines ?? [];
  const height = top + rows.length * rowH + (lines.length ? 22 + lines.length * 18 : 0) + 6;

  const drawRow = (row: StringRow, k: number) => {
    const y = top + k * rowH;
    return (
      <g key={k} data-testid={`string-row-${k}`}>
        <text x={0} y={y + size / 2 + 4} className="fill-muted-foreground text-xs font-medium">
          {t(`learn.stage.rows.${row.label}`)}
        </text>
        {row.cells.map((v, j) => {
          const role: Role = row.roles?.[j] ?? "idle";
          const x = label + (row.offset + j) * size;
          return (
            <g key={j} transform={`translate(${x},${y})`} data-role={role}>
              <rect
                x={1}
                y={0}
                width={size - 2}
                height={size}
                rx={4}
                style={{
                  fill: role === "idle" ? (row.kind === "letters" ? "var(--card)" : "transparent") : `color-mix(in srgb, ${roleColor(role)} 28%, var(--card))`,
                  stroke: role === "idle" ? (row.kind === "letters" ? "var(--viz-out)" : "transparent") : roleColor(role),
                  transition: tr.colors,
                }}
                strokeWidth={role === "idle" ? 1 : 2}
                strokeDasharray={outline(role).strokeDasharray}
              />
              <text x={size / 2} y={size / 2 + 5} textAnchor="middle" className={row.kind === "letters" ? "fill-foreground font-mono font-semibold" : "fill-foreground font-mono"} style={{ fontSize: size > 26 ? 14 : 11 }}>
                {v === null ? "" : v}
              </text>
            </g>
          );
        })}
      </g>
    );
  };

  return (
    <div ref={ref} className="relative w-full" data-testid="string-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        {Array.from({ length: widest }, (_, j) => (
          <text key={`i${j}`} x={label + j * size + size / 2} y={10} textAnchor="middle" className="fill-muted-foreground font-mono text-[10px]">
            {j}
          </text>
        ))}
        {rows.map(drawRow)}
        {lines.length > 0 && (
          <g data-testid="string-lines" transform={`translate(0,${top + rows.length * rowH + 8})`}>
            <text x={0} y={10} className="fill-muted-foreground text-xs font-medium">
              {t(`learn.stage.rows.${s!.linesLabel ?? "list"}`)}
            </text>
            {lines.map((line, k) => (
              <g key={k} transform={`translate(${label},${22 + k * 18})`} data-role={line.role ?? "idle"}>
                <text x={0} y={0} className="fill-muted-foreground font-mono text-[11px]">
                  {line.note}
                </text>
                <text x={36} y={0} className="fill-foreground font-mono text-xs" style={{ fontWeight: line.role ? 700 : 400, fill: line.role ? roleColor(line.role) : undefined }}>
                  {line.text}
                </text>
              </g>
            ))}
          </g>
        )}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
