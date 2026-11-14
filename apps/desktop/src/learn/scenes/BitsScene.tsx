// Bits (ADR 0029, Phase 1): each row is one number in binary under its place values, with
// the operator on the left and the decimal value on the right. A result row is drawn under
// a rule, like written arithmetic; a shifted row slides in from where its bits came from.
// A 1 bit is filled and says "1": the digit, not the colour, carries the value.
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import { SceneFallback, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

export const BITS = 9; // shifting an 8-bit value left needs a ninth place
const ROW_H = 46;
const TOP = 30;

export function BitsScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const rows = step.bits ?? [];
  const labelW = 104;
  const valueW = 64;
  const col = Math.max(22, Math.min(46, (width - labelW - valueW) / BITS));
  const cell = col - 6;
  const left = labelW;
  const height = TOP + Math.max(3, rows.length) * ROW_H + 8;

  return (
    <div ref={ref} className="relative w-full" data-testid="bits-scene">
      <svg
        role="img"
        aria-label={sceneSummary(t, step)}
        width={width}
        height={height}
        viewBox={`0 0 ${width} ${height}`}
        className="block overflow-visible"
      >
        {Array.from({ length: BITS }, (_, k) => (
          <text
            key={k}
            x={left + k * col + col / 2}
            y={14}
            textAnchor="middle"
            aria-hidden
            className="fill-muted-foreground font-mono text-[10px]"
          >
            {2 ** (BITS - 1 - k)}
          </text>
        ))}
        {rows.map((row, r) => {
          const y = TOP + r * ROW_H;
          const role = row.role ?? "idle";
          const result = role !== "idle";
          const from = row.shift && !instant ? row.shift * -col : 0;
          return (
            <g key={`${r}:${row.label}`} data-row={row.label} data-value={row.value} data-role={role}>
              {result && <line x1={left - 6} x2={left + BITS * col} y1={y - 5} y2={y - 5} stroke="var(--foreground)" strokeWidth={1.5} />}
              <text x={left - 12} y={y + cell / 2 + 5} textAnchor="end" className="fill-foreground font-mono text-sm font-semibold">
                {row.label}
              </text>
              <motion.g key={row.value} initial={{ x: from }} animate={{ x: 0 }} transition={tr.move}>
                {Array.from({ length: BITS }, (_, k) => {
                  const bit = (row.value >> (BITS - 1 - k)) & 1;
                  const color = result ? roleColor(role) : "var(--viz-pivot)";
                  return (
                    <g key={k} data-bit={bit}>
                      <rect
                        x={left + k * col + 3}
                        y={y}
                        width={cell}
                        height={cell}
                        rx={6}
                        style={{
                          fill: bit ? `color-mix(in srgb, ${color} 30%, var(--card))` : "var(--card)",
                          stroke: bit ? color : "var(--border)",
                          transition: tr.colors,
                        }}
                        strokeWidth={bit ? 2 : 1}
                      />
                      <text
                        x={left + k * col + 3 + cell / 2}
                        y={y + cell / 2 + 5}
                        textAnchor="middle"
                        className="font-mono text-sm"
                        style={{ fill: bit ? "var(--foreground)" : "var(--muted-foreground)", fontWeight: bit ? 700 : 400 }}
                      >
                        {bit}
                      </text>
                    </g>
                  );
                })}
              </motion.g>
              <text
                x={left + BITS * col + 10}
                y={y + cell / 2 + 5}
                className="fill-foreground font-mono text-sm font-semibold"
              >
                = {row.value}
              </text>
            </g>
          );
        })}
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
