import { useTranslation } from "react-i18next";
import type { JsonValue } from "@/bindings";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { useSetting } from "@/lib/settings";

/**
 * How the TypeScript editor checks the code (ADR 0024): "ts" is strict TypeScript, "js"
 * plain JavaScript (checked, but no implicit-any noise). Only the editor's grammar and
 * language server change; the draft, the judge and the debugger stay TypeScript.
 */
export type ScriptMode = "js" | "ts";

const parseMode = (v: JsonValue | null): ScriptMode | undefined => (v === "js" || v === "ts" ? v : undefined);

/** The mode remembered for `problem` (setting `editor.script.<problem>`, default TS). */
export function useScriptMode(problem: string) {
  return useSetting<ScriptMode>(`editor.script.${problem}`, "ts", parseMode);
}

export function ScriptModeToggle({
  mode,
  onChange,
  disabled = false,
}: {
  mode: ScriptMode;
  onChange: (mode: ScriptMode) => void;
  disabled?: boolean;
}) {
  const { t } = useTranslation();
  return (
    <ToggleGroup
      type="single"
      variant="outline"
      size="sm"
      aria-label={t("workspace.script.label")}
      title={t("workspace.script.hint")}
      value={mode}
      onValueChange={(v) => v && onChange(v as ScriptMode)}
      disabled={disabled}
      data-testid="script-mode"
    >
      <ToggleGroupItem value="js" aria-label={t("workspace.script.js")} className="px-2 text-xs">
        JS
      </ToggleGroupItem>
      <ToggleGroupItem value="ts" aria-label={t("workspace.script.ts")} className="px-2 text-xs">
        TS
      </ToggleGroupItem>
    </ToggleGroup>
  );
}
