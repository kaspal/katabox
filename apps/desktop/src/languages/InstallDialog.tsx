import { useTranslation } from "react-i18next";
import type { Language } from "@/bindings";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { InstallPanel } from "@/languages/InstallPanel";
import { installUnit } from "@/languages/useInstall";

/** "Install <language>" from the workspace, instead of failing (ADR 0015). */
export function InstallDialog({
  language,
  onOpenChange,
  onInstalled,
}: {
  language: Language | null;
  onOpenChange: (open: boolean) => void;
  /** After "Done" on a finished install. */
  onInstalled?: (language: Language) => void;
}) {
  const { t } = useTranslation();
  const unit = language ? installUnit(language) : null;
  const name = unit ? t(`install.unit.${unit as "python" | "typescript" | "php" | "rust"}`) : "";
  return (
    <Dialog open={language !== null} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>{t("install.installLanguage", { language: name })}</DialogTitle>
          <DialogDescription>{t("install.missingHint", { language: name })}</DialogDescription>
        </DialogHeader>
        {unit && language && (
          <InstallPanel
            key={unit}
            preselect={[unit]}
            onDone={() => {
              onOpenChange(false);
              onInstalled?.(language);
            }}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}
