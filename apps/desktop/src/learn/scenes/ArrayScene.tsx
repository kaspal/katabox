// Array cells (ADR 0029): values in boxes with pointers and a range bracket. A lifted
// item (insertion's key) floats above its slot, which is drawn as a dashed gap; a shift
// slides the bigger value right into the gap while the key glides left above it.
// Optional extras: a second labelled row (`aux`: prefix sums, a merge buffer, a resized
// array's new storage), empty slots up to `capacity`, a highlighted `window`, and the two
// runs of a merge. With `hidden`, the aux row shares identity with the main row: a value
// in the buffer is drawn there only, and slides back up when it is written.
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";
import type { Item, Role, Step } from "../engine/types";
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

const LIFTED_ROW_Y = 86; // cells sit here when a key can be lifted above them (y = 0)
const ROW_Y_FLAT = 14;
const BRACKET_GAP = 12;
const AUX_GAP = 56; // from the pointer row to the aux row's cells (room for its label)

export function Cell({
  item,
  role,
  cell,
  x,
  y,
  tr,
  lifted,
  keyPrefix = "",
}: {
  item: Item;
  role: Role;
  cell: number;
  x: number;
  y: number;
  tr: ReturnType<typeof useTransitions>;
  lifted?: boolean;
  keyPrefix?: string;
}) {
  return (
    <motion.g
      initial={false}
      animate={{ x, y }}
      transition={{ x: tr.move, y: tr.lift }}
      data-id={`${keyPrefix}${item.id}`}
      data-role={role}
      data-lifted={lifted || undefined}
    >
      <rect
        width={cell}
        height={cell}
        rx={Math.min(8, cell / 4)}
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
        y={cell / 2 + (cell < 30 ? 4 : 5)}
        textAnchor="middle"
        className="fill-foreground font-mono font-semibold"
        style={{ fontSize: cell < 24 ? 10 : cell < 30 ? 11 : 15 }}
      >
        {item.value}
      </text>
    </motion.g>
  );
}

/** Columns the scene spans: the main row (or its capacity) and the aux row with its offset. */
function span(step: Step): { left: number; right: number } {
  const n = Math.max(step.items.length, step.capacity ?? 0);
  let left = 0;
  let right = n;
  if (step.aux) {
    const off = step.aux.offset ?? 0;
    left = Math.min(left, off);
    right = Math.max(right, off + step.aux.cells.length);
  }
  return { left, right };
}

type Placed = { item: Item; key: string; x: number; y: number; role: Role; lifted: boolean; pos?: number };

export function ArrayScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const ROW_Y = def.lifts ? LIFTED_ROW_Y : ROW_Y_FLAT;
  const n = step.items.length;
  const { left, right } = span(step);
  const slot = width / Math.max(1, right - left);
  const x0 = -left * slot; // where position 0 of the main row starts
  const cell = Math.max(18, Math.min(60, slot - 8));
  const inset = (slot - cell) / 2;
  const bracketY = ROW_Y + cell + BRACKET_GAP;
  const indexY = bracketY + 20;
  const pointerY = indexY + 10;
  const auxY = pointerY + AUX_GAP;
  const height = step.aux ? auxY + cell + 26 : pointerY + 34;
  const hidden = new Set(step.hidden ?? []);
  const shared = step.hidden !== undefined;
  const slots = Math.max(n, step.capacity ?? 0);
  const at = (p: number) => x0 + p * slot + inset;
  const auxAt = (k: number) => x0 + ((step.aux?.offset ?? 0) + k) * slot + inset;

  // Every value drawn once: in the main row unless hidden; aux cells share its key space
  // when `hidden` is set (a merge buffer), else they are copies with their own keys.
  const placed: Placed[] = [];
  step.items.forEach((it, p) => {
    if (hidden.has(it.id)) return;
    const lifted = step.lifted === p;
    placed.push({ item: it, key: `${it.id}`, x: at(p), y: lifted ? 0 : ROW_Y, role: roleAt(step, p), lifted, pos: p });
  });
  step.aux?.cells.forEach((c, k) => {
    if (!c) return;
    placed.push({
      item: c,
      key: shared ? `${c.id}` : `aux-${c.id}`,
      x: auxAt(k),
      y: auxY,
      role: step.aux!.roles?.[k] ?? "idle",
      lifted: false,
    });
  });
  placed.sort((a, b) => (a.key < b.key ? -1 : a.key > b.key ? 1 : 0));

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
        {step.window && (
          <rect
            x={at(step.window[0]) - inset + 2}
            y={ROW_Y - 8}
            width={(step.window[1] - step.window[0] + 1) * slot - 4}
            height={cell + 16}
            rx={12}
            fill="var(--viz-range)"
            stroke="var(--viz-ptr-i)"
            strokeWidth={2}
            data-testid="window"
          />
        )}
        {/* The slots: a dashed gap where the lifted or moved-out item came from; empty capacity. */}
        {Array.from({ length: slots }, (_, p) => {
          const gap = step.lifted === p || p >= n || (p < n && hidden.has(step.items[p]!.id));
          return (
            <rect
              key={p}
              x={at(p)}
              y={ROW_Y}
              width={cell}
              height={cell}
              rx={Math.min(8, cell / 4)}
              fill="none"
              stroke="var(--border)"
              strokeWidth={gap ? 2 : 1}
              strokeDasharray={gap ? "5 4" : undefined}
              data-gap={step.lifted === p || undefined}
              data-empty={gap || undefined}
            />
          );
        })}
        {step.range && !step.runs && (
          <motion.path
            initial={false}
            animate={{
              d: `M${at(step.range[0])} ${bracketY - 5} v5 H${at(step.range[1]) + cell} v-5`,
            }}
            transition={tr.move}
            fill="none"
            stroke="var(--viz-idle)"
            strokeWidth={2}
            data-testid="range-bracket"
          />
        )}
        {step.runs &&
          step.runs.map(([lo, hi], k) => (
            <g key={k} data-run={k === 0 ? "left" : "right"}>
              <path
                d={`M${at(lo)} ${bracketY - 5} v5 H${at(hi) + cell} v-5`}
                fill="none"
                stroke={k === 0 ? "var(--viz-ptr-i)" : "var(--viz-ptr-j)"}
                strokeWidth={2.5}
              />
              <text
                x={(at(lo) + at(hi) + cell) / 2}
                y={bracketY + 9}
                textAnchor="middle"
                className="text-[10px] font-semibold"
                style={{ fill: k === 0 ? "var(--viz-ptr-i)" : "var(--viz-ptr-j)" }}
              >
                {t(k === 0 ? "learn.stage.left_run" : "learn.stage.right_run")}
              </text>
            </g>
          ))}
        {step.aux && (
          <g data-testid="aux-row">
            <text x={4} y={auxY - 10} className="fill-muted-foreground text-xs font-medium">
              {t(`learn.stage.rows.${step.aux.label}`)}
            </text>
            {step.aux.cells.map((c, k) => (
              <g key={`slot${k}`}>
                {!c && (
                  <rect
                    x={auxAt(k)}
                    y={auxY}
                    width={cell}
                    height={cell}
                    rx={Math.min(8, cell / 4)}
                    fill="none"
                    stroke="var(--border)"
                    strokeDasharray="5 4"
                    data-aux-empty={k}
                  />
                )}
                <text
                  x={auxAt(k) + cell / 2}
                  y={auxY + cell + 16}
                  textAnchor="middle"
                  aria-hidden
                  className="fill-muted-foreground font-mono text-[11px]"
                >
                  {k}
                </text>
              </g>
            ))}
          </g>
        )}
        {placed.map((c) => (
          <g key={c.key} data-pos={c.pos}>
            <Cell item={c.item} role={c.role} cell={cell} x={c.x} y={c.y} tr={tr} lifted={c.lifted} keyPrefix={c.key.startsWith("aux") ? "aux-" : ""} />
          </g>
        ))}
        <g transform={`translate(${x0} 0)`}>
          <IndexRow n={n} slot={slot} y={step.runs ? indexY + 6 : indexY} />
          <Pointers pointers={step.pointers} slot={slot} y={step.runs ? pointerY + 6 : pointerY} transition={tr.move} />
        </g>
      </svg>
      <SceneFallback t={t} step={step} def={def} />
    </div>
  );
}
