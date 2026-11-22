// Fix the bug (expansion M6, ADR 0035): the editor of a fix problem starts from buggy
// code. The banner says so, and resets the editor to that code on request.
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Bug, RotateCcw } from "lucide-react";
import type { Language, ProblemView } from "@/bindings";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

export function FixBanner({
  problem,
  language,
  onReset,
}: {
  problem: ProblemView;
  language: Language;
  onReset: () => void;
}) {
  const { t } = useTranslation();
  const [confirm, setConfirm] = useState(false);
  if (!problem.fix) return null;
  const buggy = problem.fix_languages.includes(language);
  return (
    <div
      role="status"
      data-testid="fix-banner"
      className="flex items-center gap-2 border-b bg-muted/40 px-3 py-1.5 text-sm"
    >
      <Bug aria-hidden className="size-4 shrink-0" />
      <span className="min-w-0 flex-1">{buggy ? t("fix.banner") : t("fix.noBuggy")}</span>
      {buggy && (
        <Button size="sm" variant="ghost" className="h-7 gap-1.5" onClick={() => setConfirm(true)}>
          <RotateCcw aria-hidden className="size-3.5" />
          {t("fix.reset")}
        </Button>
      )}
      <Dialog open={confirm} onOpenChange={setConfirm}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("fix.resetTitle")}</DialogTitle>
            <DialogDescription>{t("fix.resetBody")}</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setConfirm(false)}>
              {t("fix.cancel")}
            </Button>
            <Button
              data-testid="fix-reset-confirm"
              onClick={() => {
                setConfirm(false);
                onReset();
              }}
            >
              {t("fix.reset")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
