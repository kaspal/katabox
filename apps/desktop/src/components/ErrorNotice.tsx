import { AlertTriangle } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { describeError } from "@/lib/errors";

/** The one way errors are shown: translated summary, technical detail on demand. */
export function ErrorNotice({ error, onRetry }: { error: unknown; onRetry?: () => void }) {
  const { t, i18n } = useTranslation();
  const { summary, detail } = describeError(i18n, error);
  return (
    <div
      role="alert"
      className="flex items-start gap-3 rounded-lg border border-destructive/40 bg-card p-4 text-sm"
    >
      <AlertTriangle aria-hidden className="mt-0.5 size-4 shrink-0 text-destructive" />
      <div className="min-w-0 flex-1 space-y-1">
        <p className="font-medium">{summary}</p>
        {detail && detail !== summary && (
          <details className="text-muted-foreground">
            <summary className="cursor-pointer">{t("common.details")}</summary>
            <pre className="mt-1 whitespace-pre-wrap font-mono text-xs">{detail}</pre>
          </details>
        )}
      </div>
      {onRetry && (
        <Button size="sm" variant="outline" onClick={onRetry}>
          {t("common.retry")}
        </Button>
      )}
    </div>
  );
}
