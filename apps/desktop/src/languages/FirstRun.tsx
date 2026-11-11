import { useTranslation } from "react-i18next";
import { Boxes } from "lucide-react";
import { Button } from "@/components/ui/button";
import { InstallPanel } from "@/languages/InstallPanel";

/** Shown while no language is installed: choose, see sizes, install, then go Home. */
export function FirstRun({ onDone, onSkip }: { onDone: () => void; onSkip: () => void }) {
  const { t } = useTranslation();
  return (
    <div className="mx-auto max-w-3xl space-y-6 p-8" data-testid="first-run">
      <div className="space-y-2">
        <div className="flex items-center gap-3">
          <Boxes aria-hidden className="size-7 text-primary" />
          <h1 className="text-2xl font-semibold">{t("install.firstRunTitle")}</h1>
        </div>
        <p className="text-sm text-muted-foreground">{t("install.firstRunIntro")}</p>
      </div>
      <InstallPanel
        onDone={onDone}
        extraActions={
          <Button variant="ghost" onClick={onSkip}>
            {t("install.notNow")}
          </Button>
        }
      />
    </div>
  );
}
