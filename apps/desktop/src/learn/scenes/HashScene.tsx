// Hashing (ADR 0029, Phase 2). HashScene: one column per bucket; chaining stacks the
// bucket's keys downward, open addressing holds one key per slot (✕ marks a tombstone).
// The slots visited for the current key carry their visit order (1, 2, 3…), and the line
// above says how the home slot was computed. BitArrayScene: a Bloom filter's bits, with
// the positions the current key touches outlined. Both keep the output row (`aux`).
import { useTranslation } from "react-i18next";
import { Cell } from "./ArrayScene";
import { SceneFallback, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";

const CELL = 40;

function OutputRow({ step, y, width, tr }: { step: SceneProps["step"]; y: number; width: number; tr: ReturnType<typeof useTransitions> }) {
  const { t } = useTranslation();
  if (!step.aux) return null;
  const slot = Math.min(48, (width - 4) / Math.max(step.aux.cells.length, 10));
  return (
    <g data-testid="aux-row">
      <text x={0} y={y} className="fill-muted-foreground text-xs font-medium">
        {t(`learn.stage.rows.${step.aux.label}`)}
      </text>
      {step.aux.cells.map((c, k) =>
        c ? (
          <g key={`o${c.id}`}>
            <Cell item={c} role={step.aux!.roles?.[k] ?? "idle"} cell={Math.min(34, slot - 6)} x={k * slot} y={y + 8} tr={tr} keyPrefix="aux-" />
          </g>
        ) : null,
      )}
    </g>
  );
}

export function HashScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const hash = step.hash ?? { mode: "chain" as const, buckets: [], probe: [], key: null, home: null };
  const m = Math.max(1, hash.buckets.length);
  const slot = Math.min(84, width / m);
  const cell = Math.min(CELL, slot - 10);
  const top = 44;
  const deepest = hash.mode === "chain" ? Math.max(3, ...hash.buckets.map((b) => b.length)) : 1;
  const tableH = top + 22 + deepest * (cell + 6);
  const height = tableH + (step.aux ? 70 : 10);
  const visit = new Map<number, number>();
  hash.probe.forEach((b, k) => {
    if (!visit.has(b)) visit.set(b, k + 1);
  });

  return (
    <div ref={ref} className="relative w-full" data-testid="hash-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        {hash.key !== null && (
          <text x={0} y={16} className="fill-foreground font-mono text-sm font-semibold" data-testid="hash-formula">
            {t("learn.stage.hash_formula", { key: hash.key, m, home: hash.home ?? "—" })}
          </text>
        )}
        {hash.buckets.map((bucket, b) => {
          const x = b * slot + (slot - cell) / 2;
          const order = visit.get(b);
          const current = hash.probe.length > 0 && hash.probe[hash.probe.length - 1] === b;
          return (
            <g key={`b${b}`} data-bucket={b} data-visit={order}>
              <text x={x + cell / 2} y={top - 6} textAnchor="middle" className="fill-muted-foreground font-mono text-[11px]">
                {b}
              </text>
              {order !== undefined && (
                <text x={x + cell - 2} y={top - 6} textAnchor="end" className="font-mono text-[10px] font-bold" style={{ fill: "var(--viz-ptr-j)" }}>
                  {`#${order}`}
                </text>
              )}
              <rect
                x={x - 4}
                y={top}
                width={cell + 8}
                height={hash.mode === "chain" ? deepest * (cell + 6) + 8 : cell + 8}
                rx={8}
                fill={order !== undefined ? "var(--viz-range)" : "none"}
                stroke={current ? roleColor("compare") : "var(--border)"}
                strokeWidth={current ? 2.5 : 1}
                strokeDasharray={current ? "5 3" : undefined}
              />
              {bucket.map((it, k) =>
                it.value === -2 ? (
                  <text key={`t${it.id}`} x={x + cell / 2} y={top + 4 + cell / 2 + 5} textAnchor="middle" className="fill-muted-foreground font-mono text-lg" data-tombstone>
                    ✕
                  </text>
                ) : (
                  <g key={it.id}>
                    <Cell
                      item={it}
                      role={it.value === hash.key && current ? "swap" : "idle"}
                      cell={cell}
                      x={x}
                      y={top + 4 + k * (cell + 6)}
                      tr={tr}
                    />
                  </g>
                ),
              )}
            </g>
          );
        })}
        <OutputRow step={step} y={tableH + 18} width={width} tr={tr} />
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}

export function BitArrayScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const arr = step.bitArray ?? { bits: [], hits: [], miss: null };
  const m = Math.max(1, arr.bits.length);
  const slot = Math.min(46, width / m);
  const cell = slot - 6;
  const top = 34;
  const height = top + cell + 30 + (step.aux ? 70 : 0);
  const hit = new Set(arr.hits);
  return (
    <div ref={ref} className="relative w-full" data-testid="bitarray-scene">
      <svg role="img" aria-label={sceneSummary(t, step)} width={width} height={height} viewBox={`0 0 ${width} ${height}`} className="block overflow-visible">
        {arr.key !== null && arr.key !== undefined && (
          <text x={0} y={16} className="fill-foreground font-mono text-sm font-semibold">
            {t("learn.stage.bloom_key", { key: arr.key, positions: arr.hits.join(", ") || "—" })}
          </text>
        )}
        {arr.bits.map((bit, k) => {
          const on = hit.has(k);
          const miss = arr.miss === k;
          const color = miss ? roleColor("swap") : on ? roleColor("compare") : "var(--viz-pivot)";
          return (
            <g key={k} data-bit={k} data-value={bit} data-hit={on || undefined}>
              <rect
                x={k * slot + 3}
                y={top}
                width={cell}
                height={cell}
                rx={6}
                style={{ fill: bit ? `color-mix(in srgb, ${color} 30%, var(--card))` : "var(--card)", stroke: on || miss ? color : bit ? color : "var(--border)", transition: tr.colors }}
                strokeWidth={on || miss ? 2.5 : 1}
                strokeDasharray={on ? "5 3" : undefined}
              />
              <text x={k * slot + 3 + cell / 2} y={top + cell / 2 + 5} textAnchor="middle" className="font-mono text-sm" style={{ fill: bit ? "var(--foreground)" : "var(--muted-foreground)", fontWeight: bit ? 700 : 400 }}>
                {bit}
              </text>
              <text x={k * slot + 3 + cell / 2} y={top + cell + 14} textAnchor="middle" className="fill-muted-foreground font-mono text-[10px]">
                {k}
              </text>
            </g>
          );
        })}
        <OutputRow step={step} y={top + cell + 44} width={width} tr={tr} />
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
