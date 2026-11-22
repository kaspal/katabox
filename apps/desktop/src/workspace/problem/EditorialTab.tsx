import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { BookOpenCheck, Lock } from "lucide-react";
import { ErrorNotice } from "@/components/ErrorNotice";
import { Skeleton } from "@/components/ui/skeleton";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";
import { Markdown } from "@/workspace/problem/Markdown";
import { SeeItAnimated } from "@/workspace/problem/SeeItAnimated";

/** Locked until solved or after enough failed submissions (decided in kb-app). The
 * "See it animated" Learn links name the technique, so they unlock with it. */
export function EditorialTab({ problemId, pack = false }: { problemId: string; pack?: boolean }) {
  const { t } = useTranslation();
  const { lang } = usePrefs();
  const q = useQuery({
    queryKey: ["editorial", problemId, lang],
    queryFn: () => api.editorial(problemId, lang),
  });
  if (q.isPending) return <Skeleton className="h-24 w-full" />;
  if (q.error) return <ErrorNotice error={q.error} onRetry={() => void q.refetch()} />;
  const e = q.data;
  if (!e.available) return <p className="text-sm text-muted-foreground">{t("workspace.editorial.none")}</p>;
  if (!e.unlocked) {
    return (
      <div className="flex items-start gap-3 rounded-lg border border-dashed p-4" data-testid="editorial-locked">
        <Lock aria-hidden className="mt-0.5 size-5 text-muted-foreground" />
        <div className="space-y-1 text-sm">
          <p className="font-medium">{t("workspace.editorial.locked")}</p>
          <p className="text-muted-foreground">
            {t("workspace.editorial.lockedReason", {
              failed: e.failed_submits,
              required: e.required_failures,
            })}
          </p>
        </div>
      </div>
    );
  }
  return (
    <div className="space-y-3" data-testid="editorial-unlocked">
      <p className="flex items-center gap-2 text-xs text-muted-foreground">
        <BookOpenCheck aria-hidden className="size-4" />
        {e.reason === "accepted" ? t("workspace.editorial.unlockedSolved") : t("workspace.editorial.unlockedAttempts")}
      </p>
      <SeeItAnimated problemId={problemId} />
      <Markdown text={e.text ?? ""} trust={pack ? "pack" : "app"} />
    </div>
  );
}
