import { useTranslation } from "react-i18next";
import {
  AlertCircle,
  AlertTriangle,
  CircleCheck,
  CircleDashed,
  CircleSlash,
  Download,
  Loader2,
  PauseCircle,
  WandSparkles,
} from "lucide-react";
import type { Language } from "@/bindings";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";
import type { EditorStatus } from "@/workspace/editor/CodeEditor";
import type { ServerState } from "@/workspace/editor/lsp/useLanguageServer";

/** Server status, problems count and cursor position under the editor. */
export function EditorStatusBar({
  language,
  server,
  status,
  onFormat,
  onInstall,
  basicCompletion = false,
}: {
  language: Language;
  server: ServerState;
  status: EditorStatus | null;
  onFormat: () => void;
  /** Offered when the server is unavailable because it is not installed. */
  onInstall?: () => void;
  /** A static completion list stands in for the missing server (PHP). */
  basicCompletion?: boolean;
}) {
  const { t } = useTranslation();
  const name = t(`languages.${language}`);
  const text = (() => {
    switch (server.state) {
      case "off":
        return t("lsp.status.off");
      case "connecting":
      case "starting":
        return t("lsp.status.starting", { language: name });
      case "ready":
        return t("lsp.status.ready", { language: name });
      case "restarting":
        return t("lsp.status.restarting", { language: name, attempt: server.attempt });
      case "stopped":
        return t("lsp.status.stopped", { language: name });
      case "unavailable":
        return t("lsp.status.unavailable", { language: name });
    }
  })();
  const Icon =
    server.state === "ready"
      ? CircleCheck
      : server.state === "connecting" || server.state === "starting" || server.state === "restarting"
        ? Loader2
        : server.state === "stopped"
          ? PauseCircle
          : server.state === "unavailable"
            ? CircleSlash
            : CircleDashed;
  const reason = server.state === "unavailable" ? server.reason : null;
  const label = (
    <span className="inline-flex items-center gap-1.5" data-testid="lsp-status" data-state={server.state}>
      <Icon
        aria-hidden
        className={cn(
          "size-3.5",
          server.state === "ready" && "text-verdict-accepted",
          Icon === Loader2 && "animate-spin",
        )}
      />
      {text}
      {basicCompletion && (
        <span data-testid="basic-completion"> · {t("lsp.status.basicCompletion")}</span>
      )}
    </span>
  );
  return (
    <div
      className="flex h-7 shrink-0 items-center gap-4 border-t bg-muted/40 px-3 text-xs text-muted-foreground"
      role="status"
      aria-live="polite"
    >
      {reason ? (
        <Tooltip>
          <TooltipTrigger asChild>
            <span tabIndex={0} className="cursor-help">
              {label}
            </span>
          </TooltipTrigger>
          <TooltipContent className="max-w-md">{t("lsp.reason", { reason })}</TooltipContent>
        </Tooltip>
      ) : (
        label
      )}
      {onInstall && reason === "not installed" && (
        <Button variant="ghost" size="sm" className="h-6 gap-1 px-2 text-xs" onClick={onInstall}>
          <Download aria-hidden className="size-3.5" />
          {t("install.install")}
        </Button>
      )}
      {status && (
        <span className="inline-flex items-center gap-3" data-testid="problems-count">
          <span className={cn("inline-flex items-center gap-1", status.errors > 0 && "text-destructive")}>
            <AlertCircle aria-hidden className="size-3.5" />
            {t("lsp.errors", { count: status.errors })}
          </span>
          <span className={cn("inline-flex items-center gap-1", status.warnings > 0 && "text-difficulty-medium")}>
            <AlertTriangle aria-hidden className="size-3.5" />
            {t("lsp.warnings", { count: status.warnings })}
          </span>
        </span>
      )}
      <span className="ml-auto inline-flex items-center gap-3">
        {status && <span className="tabular-nums">{t("lsp.position", { line: status.line, col: status.col })}</span>}
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="h-6 gap-1 px-2 text-xs"
              onClick={onFormat}
              disabled={server.state !== "ready"}
              aria-keyshortcuts="Shift+Alt+F"
            >
              <WandSparkles aria-hidden className="size-3.5" />
              {t("lsp.format")}
            </Button>
          </TooltipTrigger>
          <TooltipContent>{t("lsp.formatHint")}</TooltipContent>
        </Tooltip>
      </span>
    </div>
  );
}
