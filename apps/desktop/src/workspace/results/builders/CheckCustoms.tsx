// "Check" for custom tests (expansion M9): the signature's types and the problem's
// validate.py, in the sandbox, without running any solution.
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { CircleCheck, CircleX, ListChecks, Loader2 } from "lucide-react";
import type { CustomCheck } from "@/bindings";
import { Button } from "@/components/ui/button";
import { ErrorNotice } from "@/components/ErrorNotice";
import { api } from "@/lib/api";
import { validateCustom, type CustomTestDraft, type Signature } from "@/workspace/results/validate";

export function CheckCustoms({
  problemId,
  signature,
  customs,
  disabled,
}: {
  problemId: string;
  signature: Signature;
  customs: CustomTestDraft[];
  disabled: boolean;
}) {
  const { t } = useTranslation();
  const [state, setState] = useState<
    { kind: "idle" } | { kind: "running" } | { kind: "done"; result: CustomCheck; indexes: number[] } | { kind: "error"; error: unknown }
  >({ kind: "idle" });
  // Only inputs that already parse as their types go to the backend; the others show
  // their error under the field.
  const ready = customs
    .map((c, k) => ({ k, input: validateCustom(signature, c.values).input }))
    .filter((x) => x.input !== null);
  const check = async () => {
    setState({ kind: "running" });
    try {
      const result = await api.checkCustom(problemId, ready.map((x) => x.input!));
      setState({ kind: "done", result, indexes: ready.map((x) => x.k + 1) });
    } catch (error) {
      setState({ kind: "error", error });
    }
  };
  return (
    <div className="space-y-2">
      <Button
        size="sm"
        variant="outline"
        onClick={() => void check()}
        disabled={disabled || ready.length === 0 || state.kind === "running"}
        data-testid="check-customs"
      >
        {state.kind === "running" ? <Loader2 aria-hidden className="animate-spin" /> : <ListChecks aria-hidden />}
        {t("builders.check")}
      </Button>
      {state.kind === "error" && <ErrorNotice error={state.error} />}
      {state.kind === "done" && (
        <ul role="status" className="space-y-1 text-xs" data-testid="check-results">
          {state.result.problems.map((p, i) => (
            <li key={i} className="flex items-start gap-1.5">
              {p === null ? (
                <CircleCheck aria-hidden className="mt-0.5 size-3.5 flex-none text-verdict-accepted" />
              ) : (
                <CircleX aria-hidden className="mt-0.5 size-3.5 flex-none text-verdict-wrong" />
              )}
              <span>
                {p === null
                  ? t("builders.checkOk", { index: state.indexes[i] })
                  : t("builders.checkBad", { index: state.indexes[i], why: p })}
              </span>
            </li>
          ))}
          {!state.result.constraints_checked && (
            <li className="text-muted-foreground">{t("builders.checkTypesOnly")}</li>
          )}
        </ul>
      )}
    </div>
  );
}
