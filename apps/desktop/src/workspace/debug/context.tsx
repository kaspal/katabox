// "Debug this case" (ADR 0022): the workspace provides how to start a debug session on
// a test; test cards and the first-failure view use it without prop drilling.
import { createContext, useContext } from "react";
import type { CustomInput } from "@/lib/api";
import { DebugCaseButton } from "@/workspace/debug/DebugPanel";

export type DebugCase = {
  onDebug: (input: CustomInput, label: string) => void;
  /** Why debugging is unavailable (another language, a judge run going on), or null. */
  disabledReason: string | null;
};

export const DebugCaseContext = createContext<DebugCase | null>(null);

/** The "Debug this case" button for one test, or nothing outside a workspace. */
export function DebugThisCase({ input, label }: { input: CustomInput | null; label: string }) {
  const ctx = useContext(DebugCaseContext);
  if (!ctx || input === null) return null;
  return <DebugCaseButton onDebug={() => ctx.onDebug(input, label)} disabledReason={ctx.disabledReason} />;
}
