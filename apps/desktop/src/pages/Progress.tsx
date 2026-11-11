import { useTranslation } from "react-i18next";
import { BarChart3 } from "lucide-react";
import { Page } from "@/components/Page";

/** Placeholder: statistics and streaks come later. */
export function Progress() {
  const { t } = useTranslation();
  return (
    <Page title={t("progress.title")}>
      <div className="flex max-w-3xl items-center gap-3 rounded-lg border border-dashed bg-card p-5 text-muted-foreground">
        <BarChart3 aria-hidden className="size-5" />
        <p>{t("progress.comingSoon")}</p>
      </div>
    </Page>
  );
}
