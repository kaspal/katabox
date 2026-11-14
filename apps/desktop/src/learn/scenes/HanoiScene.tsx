// Towers of Hanoi (ADR 0029, Phase 1): three pegs with disks keyed by size, so a move
// lifts the disk, carries it over and drops it; the call stack on the right shows which
// call is moving it. The disk being moved is outlined and named in the narration.
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import { SceneFallback, outline, roleColor, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";
import { StackColumn } from "./parts";

const DISK_H = 22;
const BASE_PAD = 34;

export function HanoiScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const pegs = step.pegs ?? [[], [], []];
  const disks = pegs.reduce((m, p) => m + p.length, 0);
  const stackW = Math.min(230, Math.max(150, width * 0.32));
  const areaW = Math.max(180, width - stackW - 24);
  const pegW = areaW / 3;
  const maxDiskW = Math.min(150, pegW - 12);
  const frames = disks + 1;
  const pegTop = 40;
  const base = pegTop + Math.max(disks, 3) * DISK_H + 26;
  const stackBottom = Math.max(base, 28 + frames * 36 + 6);
  const height = Math.max(base + BASE_PAD, stackBottom + 14);
  const moving = step.active ?? undefined; // the disk being moved
  const diskW = (size: number) => 26 + ((maxDiskW - 26) * size) / Math.max(1, disks);

  const where = new Map<number, { peg: number; level: number }>();
  pegs.forEach((p, k) => p.forEach((size, level) => where.set(size, { peg: k, level })));
  const sizes = [...where.keys()].sort((a, b) => a - b);

  return (
    <div ref={ref} className="relative w-full" data-testid="hanoi-scene">
      <svg
        role="img"
        aria-label={sceneSummary(t, step)}
        width={width}
        height={height}
        viewBox={`0 0 ${width} ${height}`}
        className="block overflow-visible"
      >
        <line x1={4} x2={areaW} y1={base} y2={base} stroke="var(--viz-edge)" strokeWidth={3} />
        {[0, 1, 2].map((k) => (
          <g key={k}>
            <rect x={(k + 0.5) * pegW - 3} y={pegTop} width={6} height={base - pegTop} rx={3} fill="var(--viz-edge)" />
            <text
              x={(k + 0.5) * pegW}
              y={base + 22}
              textAnchor="middle"
              className="fill-foreground font-mono text-sm font-semibold"
            >
              {t("learn.stage.peg", { peg: k + 1 })}
            </text>
          </g>
        ))}
        {sizes.map((size) => {
          const { peg, level } = where.get(size)!;
          const w = diskW(size);
          const active = moving === size;
          const role = active ? "swap" : "idle";
          return (
            <motion.g
              key={size}
              initial={false}
              animate={{ x: (peg + 0.5) * pegW - w / 2, y: base - (level + 1) * DISK_H }}
              transition={tr.move}
              data-disk={size}
              data-peg={peg + 1}
            >
              <rect
                width={w}
                height={DISK_H - 3}
                rx={7}
                style={{
                  fill: active
                    ? `color-mix(in srgb, ${roleColor(role)} 45%, var(--card))`
                    : `color-mix(in srgb, var(--viz-pivot) ${18 + size * 8}%, var(--card))`,
                  stroke: active ? roleColor(role) : "var(--viz-pivot)",
                  transition: tr.colors,
                }}
                strokeWidth={active ? 2.5 : 1.5}
                strokeDasharray={outline(role).strokeDasharray}
              />
              <text x={w / 2} y={DISK_H / 2 + 2} textAnchor="middle" className="fill-foreground font-mono text-xs font-bold">
                {size}
              </text>
            </motion.g>
          );
        })}
        <StackColumn
          entries={step.stack ?? []}
          x={width - stackW}
          width={stackW}
          bottom={stackBottom}
          title={t("learn.stage.call_stack")}
          empty={t("learn.stage.stack_empty")}
          tr={tr}
        />
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
