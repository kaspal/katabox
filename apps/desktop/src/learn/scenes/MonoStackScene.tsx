// Monotonic stack (ADR 0029, Phase 1): the array with its answer row (ArrayScene, which
// also writes the text fallback, stack included) and, beside it, the stack column: the
// positions still waiting for a bigger value, smallest on top.
import { useTranslation } from "react-i18next";
import { ArrayScene } from "./ArrayScene";
import { useTransitions, type SceneProps } from "./common";
import { StackColumn, stackHeight } from "./parts";

const STACK_W = 132;

export function MonoStackScene(props: SceneProps) {
  const { t } = useTranslation();
  const tr = useTransitions(props.instant);
  const entries = props.step.stack ?? [];
  const depth = Math.max(2, props.step.stackDepth ?? props.step.items.length);
  const height = stackHeight(depth);
  return (
    <div className="flex w-full items-start gap-4" data-testid="monostack-scene">
      <div className="min-w-0 flex-1">
        <ArrayScene {...props} />
      </div>
      <svg width={STACK_W} height={height} viewBox={`0 0 ${STACK_W} ${height}`} aria-hidden className="flex-none overflow-visible">
        <StackColumn
          entries={entries}
          x={0}
          width={STACK_W - 4}
          bottom={height - 14}
          title={t("learn.stage.stack")}
          empty={t("learn.stage.stack_empty")}
          tr={tr}
        />
      </svg>
    </div>
  );
}
