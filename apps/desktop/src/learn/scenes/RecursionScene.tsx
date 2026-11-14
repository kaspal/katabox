// Recursion (ADR 0029, Phase 1): the array on top with the current call's range, the
// recursion tree growing below it (a range call sits over its range), and the call stack
// on the right: frames push and pop, the running one on top.
import { useTranslation } from "react-i18next";
import { Cell } from "./ArrayScene";
import { SceneFallback, roleAt, sceneSummary, useTransitions, useWidth, type SceneProps } from "./common";
import { StackColumn, TreeView } from "./parts";

const TOP = 26;
const LEVEL = 58;

export function RecursionScene({ step, instant, def }: SceneProps) {
  const { t } = useTranslation();
  const [ref, width] = useWidth<HTMLDivElement>();
  const tr = useTransitions(instant);
  const stackW = Math.min(230, Math.max(150, width * 0.3));
  const treeW = Math.max(120, width - stackW - 20);
  const n = step.items.length;
  const slot = treeW / Math.max(1, n);
  const cell = Math.max(18, Math.min(44, slot - 8));
  const treeTop = TOP + cell + 30;
  const depth = step.treeSize?.depth ?? 0;
  const frames = depth + 1;
  const treeBottom = treeTop + (depth + 1) * LEVEL;
  const stackBottom = Math.max(treeBottom - 20, 28 + frames * 36 + 6);
  const height = Math.max(treeBottom, stackBottom + 14);

  return (
    <div ref={ref} className="relative w-full" data-testid="recursion-scene">
      <svg
        role="img"
        aria-label={sceneSummary(t, step)}
        width={width}
        height={height}
        viewBox={`0 0 ${width} ${height}`}
        className="block overflow-visible"
      >
        {step.range && (
          <rect
            x={step.range[0] * slot + 2}
            y={TOP - 6}
            width={(step.range[1] - step.range[0] + 1) * slot - 4}
            height={cell + 12}
            rx={10}
            fill="var(--viz-range)"
            data-testid="call-range"
          />
        )}
        {step.items.map((it, p) => (
          <Cell key={it.id} item={it} role={roleAt(step, p)} cell={cell} x={p * slot + (slot - cell) / 2} y={TOP} tr={tr} />
        ))}
        <TreeView
          nodes={step.tree ?? []}
          columns={step.treeSize?.columns ?? 1}
          left={0}
          width={treeW}
          top={treeTop}
          level={LEVEL}
          tr={tr}
        />
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
