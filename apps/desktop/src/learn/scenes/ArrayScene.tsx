// Array cells (ADR 0029): values in boxes with pointers and a range bracket. A lifted
// item (insertion's key) floats above its slot, which is drawn as a dashed gap; a shift
// slides the bigger value right into the gap while the key glides left above it.
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

const ROW_Y = 86; // cells sit here; a lifted cell sits at y = 0
const BRACKET_GAP = 12;

export function ArrayScene({ step, instant, movesLabel }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const n = step.items.length;
  const slot = width / Math.max(1, n);
  const cell = Math.max(18, Math.min(60, slot - 8));
  const inset = (slot - cell) / 2;
  const bracketY = ROW_Y + cell + BRACKET_GAP;
  const indexY = bracketY + 20;
  const pointerY = indexY + 10;
  const height = pointerY + 34;
  const positions = new Map(step.items.map((it, p) => [it.id, p]));
  const byId = [...step.items].sort((a, b) => a.id - b.id);

  return (
    <div ref={ref} className="relative w-full" data-testid="array-scene">
      <svg
        role="img"
        aria-label={sceneSummary(t, step)}
        width={width}
        height={height}
        viewBox={`0 0 ${width} ${height}`}
        className="block overflow-visible"
      >
        {/* The slots: a dashed gap where the lifted item came from. */}
        {Array.from({ length: n }, (_, p) => (
          <rect
            key={p}
            x={p * slot + inset}
            y={ROW_Y}
            width={cell}
            height={cell}
            rx={8}
            fill="none"
            stroke="var(--border)"
            strokeWidth={step.lifted === p ? 2 : 1}
            strokeDasharray={step.lifted === p ? "5 4" : undefined}
            data-gap={step.lifted === p || undefined}
          />
        ))}
        {step.range && (
          <motion.path
            initial={false}
            animate={{
              d: `M${step.range[0] * slot + inset} ${bracketY - 5} v5 H${(step.range[1] + 1) * slot - inset} v-5`,
            }}
            transition={tr.move}
            fill="none"
            stroke="var(--viz-idle)"
            strokeWidth={2}
            data-testid="range-bracket"
          />
        )}
        {byId.map((it) => {
          const p = positions.get(it.id)!;
          const role = roleAt(step, p);
          const lifted = step.lifted === p;
          return (
            <motion.g
              key={it.id}
              initial={false}
              animate={{ x: p * slot + inset, y: lifted ? 0 : ROW_Y }}
              transition={{ x: tr.move, y: tr.lift }}
              data-id={it.id}
              data-pos={p}
              data-role={role}
              data-lifted={lifted || undefined}
            >
              <rect
                width={cell}
                height={cell}
                rx={8}
                style={{
                  fill: `color-mix(in srgb, ${roleColor(role)} 22%, var(--card))`,
                  stroke: roleColor(role),
                  transition: tr.colors,
                }}
                strokeWidth={role === "idle" || role === "out" ? 1.5 : 2.5}
                strokeDasharray={outline(role).strokeDasharray}
              />
              <text
                x={cell / 2}
                y={cell / 2 + 5}
                textAnchor="middle"
                className="fill-foreground font-mono text-[15px] font-semibold"
              >
                {it.value}
              </text>
            </motion.g>
          );
        })}
        <IndexRow n={n} slot={slot} y={indexY} />
        <Pointers pointers={step.pointers} slot={slot} y={pointerY} transition={tr.move} />
      </svg>
      <SceneFallback t={t} step={step} movesLabel={movesLabel} />
    </div>
  );
}
