// Bars for sorting topics (ADR 0029): each value is a bar keyed by its id, so swaps slide;
// active bars lift and get an outline; the active range sits on a band behind them.
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import {
  IndexRow,
  Pointers,
  SceneFallback,
  outline,
  roleAt,
  roleColor,
  sceneSummary,
  useTransitions,
  useWidth,
  type SceneProps,
} from "./common";

const BARS_H = 250; // tallest bar plus its label and lift
const MAX_BAR = 210;
const LIFT = 8;
const INDEX_Y = BARS_H + 16;
const POINTER_Y = BARS_H + 26;
export const BARS_SCENE_HEIGHT = POINTER_Y + 34;

export function BarsScene({ step, instant, movesLabel, maxValue }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const n = step.items.length;
  const slot = width / Math.max(1, n);
  const pad = Math.min(6, slot * 0.12);
  const barW = Math.max(4, slot - 2 * pad);
  const positions = new Map(step.items.map((it, p) => [it.id, p]));
  // Draw in id order so React keeps each bar's element as it moves.
  const byId = [...step.items].sort((a, b) => a.id - b.id);

  return (
    <div ref={ref} className="relative w-full" data-testid="bars-scene">
      <svg
        role="img"
        aria-label={sceneSummary(t, step)}
        width={width}
        height={BARS_SCENE_HEIGHT}
        viewBox={`0 0 ${width} ${BARS_SCENE_HEIGHT}`}
        className="block overflow-visible"
      >
        <motion.rect
          initial={false}
          animate={{
            x: step.range ? step.range[0] * slot : 0,
            width: step.range ? (step.range[1] - step.range[0] + 1) * slot : 0,
            opacity: step.range ? 1 : 0,
          }}
          transition={tr.move}
          y={0}
          height={BARS_H}
          rx={10}
          fill="var(--viz-range)"
          data-testid="range-band"
        />
        {byId.map((it) => {
          const p = positions.get(it.id)!;
          const role = roleAt(step, p);
          const active = role === "compare" || role === "swap";
          const h = Math.max(6, (it.value / Math.max(1, maxValue)) * MAX_BAR);
          return (
            <motion.g
              key={it.id}
              initial={false}
              animate={{ x: p * slot, y: active ? -LIFT : 0 }}
              transition={{ x: tr.move, y: tr.lift }}
              data-id={it.id}
              data-pos={p}
              data-role={role}
            >
              <text
                x={slot / 2}
                y={BARS_H - h - 8}
                textAnchor="middle"
                className={role === "out" ? "fill-muted-foreground text-[13px] font-semibold" : "fill-foreground text-[13px] font-semibold"}
              >
                {it.value}
              </text>
              <rect
                x={pad}
                y={BARS_H - h}
                width={barW}
                height={h}
                rx={Math.min(6, barW / 3)}
                style={{ fill: roleColor(role), transition: tr.colors }}
                {...outline(role)}
              />
            </motion.g>
          );
        })}
        <IndexRow n={n} slot={slot} y={INDEX_Y} />
        <Pointers pointers={step.pointers} slot={slot} y={POINTER_Y} transition={tr.move} />
      </svg>
      <SceneFallback t={t} step={step} movesLabel={movesLabel} />
    </div>
  );
}
