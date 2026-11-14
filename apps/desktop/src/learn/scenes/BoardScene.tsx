// Boards (ADR 0029, Phase 9): N-Queens, Sudoku, word search. A square grid of cells with a
// symbol each (Q, a digit, a letter); cells under attack are hatched, Sudoku boxes get
// thicker lines, a word-search path is a line through cell centres, and givens are bold.
// Under the board, the decision outline (DecisionScene) when the tracer records one.
import { useTranslation } from "react-i18next";
import type { Role } from "../engine/types";
import { DecisionOutline } from "./DecisionScene";
import { SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

export function BoardScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const b = step.board;
  const rows = b?.rows ?? 0;
  const cols = b?.cols ?? 0;
  const size = Math.max(22, Math.min(46, Math.floor(Math.min(width - 8, 420) / Math.max(1, cols))));
  const left = Math.max(0, (width - cols * size) / 2);
  const height = rows * size + 8;
  const shade = new Set(b?.shade ?? []);
  const given = new Set(b?.given ?? []);
  const center = (k: number) => ({ x: left + (k % cols) * size + size / 2, y: 4 + Math.floor(k / cols) * size + size / 2 });
  return (
    <div ref={ref} className="relative flex w-full flex-col gap-3" data-testid="board-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        <defs>
          <pattern id="kb-hatch" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">
            <line x1="0" y1="0" x2="0" y2="6" stroke="var(--viz-out)" strokeWidth="3" />
          </pattern>
        </defs>
        {b?.cells.map((text, k) => {
          const r = Math.floor(k / cols);
          const c = k % cols;
          const role: Role = b.roles[k] ?? "idle";
          const light = (r + c) % 2 === 0;
          return (
            <g key={k} transform={`translate(${left + c * size},${4 + r * size})`} data-cell={`${r}-${c}`} data-role={role}>
              <rect
                width={size}
                height={size}
                style={{
                  fill: role !== "idle" ? `color-mix(in srgb, ${roleColor(role)} 30%, var(--card))` : b.checker && !light ? "var(--viz-range)" : "var(--card)",
                  stroke: role === "idle" ? "var(--viz-out)" : roleColor(role),
                  transition: tr.colors,
                }}
                strokeWidth={role === "idle" ? 1 : 2.5}
                strokeDasharray={outline(role).strokeDasharray}
              />
              {shade.has(k) && role === "idle" && <rect width={size} height={size} fill="url(#kb-hatch)" opacity={0.7} />}
              {text && (
                <text x={size / 2} y={size / 2 + 5} textAnchor="middle" className="fill-foreground font-mono" style={{ fontSize: size > 34 ? 16 : 12, fontWeight: given.has(k) || text === "Q" ? 700 : 400 }}>
                  {text}
                </text>
              )}
            </g>
          );
        })}
        {b?.boxes &&
          Array.from({ length: rows / b.boxes + 1 }, (_, j) => (
            <g key={`bx${j}`}>
              <line x1={left} x2={left + cols * size} y1={4 + j * b.boxes! * size} y2={4 + j * b.boxes! * size} stroke="var(--foreground)" strokeWidth={2} />
              <line x1={left + j * b.boxes! * size} x2={left + j * b.boxes! * size} y1={4} y2={4 + rows * size} stroke="var(--foreground)" strokeWidth={2} />
            </g>
          ))}
        {b && b.path && b.path.length > 1 && (
          <polyline data-testid="board-path" points={b.path.map((k) => `${center(k).x},${center(k).y}`).join(" ")} fill="none" stroke={roleColor("swap")} strokeWidth={3} strokeLinecap="round" strokeLinejoin="round" opacity={0.85} />
        )}
      </svg>
      {step.decision && <DecisionOutline data={step.decision} height={180} />}
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
