// Choose languages, see sizes, install with per-item progress, cancel, retry; and, in
// Settings, uninstall (ADR 0015). Used by the first-run screen, Settings → Languages and
// the workspace's install dialog.
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { CheckCircle2, CircleSlash, HardDrive, Loader2, Trash2 } from "lucide-react";
import { useQueryClient } from "@tanstack/react-query";
import type { Language, LanguageStatus } from "@/bindings";
import { ErrorNotice } from "@/components/ErrorNotice";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Progress } from "@/components/ui/progress";
import { Skeleton } from "@/components/ui/skeleton";
import { api } from "@/lib/api";
import { queryKeys, useLanguages } from "@/lib/queries";
import { formatBytes, useInstall, type ItemProgress } from "@/languages/useInstall";

type Unit = "python" | "typescript" | "php" | "rust";
const asUnit = (l: Language): Unit => (l === "javascript" ? "typescript" : (l as Unit));

export function InstallPanel({
  preselect = [],
  allowUninstall = false,
  onDone,
  extraActions,
}: {
  /** Languages checked at first (still installable ones only). */
  preselect?: Language[];
  /** Show Uninstall for installed languages (Settings). */
  allowUninstall?: boolean;
  /** Shown as a "Done" button once an install finished. */
  onDone?: () => void;
  /** More buttons next to Install (e.g. "Not now" on the first-run screen). */
  extraActions?: React.ReactNode;
}) {
  const { t, i18n } = useTranslation();
  const langs = useLanguages();
  const install = useInstall();
  const [picked, setPicked] = useState<Set<Language>>(() => new Set(preselect.map(asUnit)));
  const size = (n: number) => formatBytes(n, i18n.language);
  const running = install.state === "running";

  const statuses = langs.data?.languages ?? [];
  const selectable = (s: LanguageStatus) => s.available && !s.installed;
  const chosen = statuses.filter((s) => selectable(s) && picked.has(s.language));
  const totals = useMemo(
    () => ({
      download: chosen.reduce((a, s) => a + s.download_bytes, 0),
      disk: chosen.reduce((a, s) => a + s.installed_bytes, 0),
    }),
    [chosen],
  );

  if (langs.isPending) return <Skeleton className="h-48 w-full" />;
  if (langs.error) return <ErrorNotice error={langs.error} onRetry={() => void langs.refetch()} />;
  if (!langs.data.platform_supported) {
    return <p className="text-sm text-muted-foreground">{t("install.unsupportedPlatform")}</p>;
  }

  const toggle = (l: Language, on: boolean) =>
    setPicked((prev) => {
      const next = new Set(prev);
      if (on) next.add(l);
      else next.delete(l);
      return next;
    });

  return (
    <div className="space-y-4" data-testid="install-panel">
      <ul className="grid gap-3 sm:grid-cols-2">
        {statuses.map((s) => (
          <LanguageCard
            key={s.language}
            status={s}
            checked={picked.has(s.language) && selectable(s)}
            disabled={running || !selectable(s)}
            onCheck={(on) => toggle(s.language, on)}
            allowUninstall={allowUninstall && !running}
          />
        ))}
      </ul>

      <div className="flex flex-wrap items-center gap-x-6 gap-y-1 text-xs text-muted-foreground">
        {langs.data.free_bytes !== null && (
          <span className="inline-flex items-center gap-1.5">
            <HardDrive aria-hidden className="size-3.5" />
            {t("install.freeSpace", { size: size(langs.data.free_bytes) })}
          </span>
        )}
        {chosen.length > 0 && (
          <span data-testid="install-total">
            {t("install.selected", { download: size(totals.download), disk: size(totals.disk) })}
          </span>
        )}
      </div>

      {install.items.length > 0 && <ProgressList items={install.items} />}

      {install.state === "cancelled" && (
        <p role="status" className="text-sm text-muted-foreground">
          {t("install.cancelled")}
        </p>
      )}
      {install.state === "failed" && (
        <div className="space-y-2">
          <ErrorNotice error={install.error} />
          <p className="text-sm text-muted-foreground">{t("install.failed")}</p>
        </div>
      )}
      {install.state === "done" && (
        <p role="status" className="inline-flex items-center gap-1.5 text-sm text-verdict-accepted">
          <CheckCircle2 aria-hidden className="size-4" />
          {t("install.success")}
        </p>
      )}

      <div className="flex flex-wrap items-center gap-2">
        {running ? (
          <>
            <Button disabled className="gap-2">
              <Loader2 aria-hidden className="animate-spin" />
              {t("install.install")}
            </Button>
            <Button variant="outline" onClick={install.cancel}>
              {t("install.cancel")}
            </Button>
          </>
        ) : install.state === "cancelled" || install.state === "failed" ? (
          <Button onClick={install.retry}>{t("install.retry")}</Button>
        ) : install.state === "done" && onDone ? (
          <Button onClick={onDone}>{t("install.done")}</Button>
        ) : (
          <Button disabled={chosen.length === 0} onClick={() => void install.start(chosen.map((s) => s.language))}>
            {t("install.installSelected")}
          </Button>
        )}
        {extraActions}
      </div>

      <p className="text-xs text-muted-foreground">{t("install.licenses")}</p>
    </div>
  );
}

function LanguageCard({
  status: s,
  checked,
  disabled,
  onCheck,
  allowUninstall,
}: {
  status: LanguageStatus;
  checked: boolean;
  disabled: boolean;
  onCheck: (on: boolean) => void;
  allowUninstall: boolean;
}) {
  const { t, i18n } = useTranslation();
  const unit = asUnit(s.language);
  const id = `install-${unit}`;
  const size = (n: number) => formatBytes(n, i18n.language);
  return (
    <li
      className="flex items-start gap-3 rounded-lg border bg-card p-4"
      data-testid={`language-${unit}`}
      data-installed={s.installed || undefined}
    >
      {s.installed ? (
        // Installed: nothing to select (Uninstall is below); keep the column aligned.
        <span aria-hidden className="mt-0.5 size-4 shrink-0" />
      ) : (
        <Checkbox
          id={id}
          checked={checked}
          disabled={disabled}
          onCheckedChange={(v) => onCheck(v === true)}
          aria-describedby={`${id}-details`}
          className="mt-0.5"
        />
      )}
      <div className="min-w-0 flex-1 space-y-1">
        <label htmlFor={id} className="font-medium">
          {t(`install.unit.${unit}`)}
        </label>
        <p id={`${id}-details`} className="text-xs text-muted-foreground">
          {t(`install.includes.${unit}`)}
          <br />
          {s.installed
            ? t("install.onDisk", { disk: size(s.installed_bytes) })
            : t("install.sizes", { download: size(s.download_bytes), disk: size(s.installed_bytes) })}
        </p>
        {!s.available ? (
          <p className="inline-flex items-center gap-1 text-xs text-muted-foreground">
            <CircleSlash aria-hidden className="size-3.5" />
            {t("install.unavailable")}
          </p>
        ) : s.installed ? (
          <div className="flex items-center gap-3">
            <span className="inline-flex items-center gap-1 text-xs text-verdict-accepted">
              <CheckCircle2 aria-hidden className="size-3.5" />
              {t("install.installed")}
            </span>
            {allowUninstall && <UninstallButton language={s.language} />}
          </div>
        ) : null}
      </div>
    </li>
  );
}

function UninstallButton({ language }: { language: Language }) {
  const { t } = useTranslation();
  const qc = useQueryClient();
  const [open, setOpen] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<unknown>(null);
  const name = t(`install.unit.${asUnit(language)}`);
  const confirm = async () => {
    setBusy(true);
    setError(null);
    try {
      await api.uninstallLanguage(language);
      setOpen(false);
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
      void qc.invalidateQueries({ queryKey: queryKeys.languages });
      void qc.invalidateQueries({ queryKey: queryKeys.appInfo });
    }
  };
  return (
    <>
      <Button variant="ghost" size="sm" className="h-6 gap-1 px-2 text-xs" onClick={() => setOpen(true)}>
        <Trash2 aria-hidden className="size-3.5" />
        {t("install.uninstall")}
      </Button>
      <Dialog open={open} onOpenChange={(o) => !busy && setOpen(o)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("install.uninstallTitle", { language: name })}</DialogTitle>
            <DialogDescription>{t("install.uninstallHint")}</DialogDescription>
          </DialogHeader>
          {error !== null && <ErrorNotice error={error} />}
          <DialogFooter>
            <Button variant="outline" onClick={() => setOpen(false)} disabled={busy}>
              {t("install.cancel")}
            </Button>
            <Button variant="destructive" onClick={() => void confirm()} disabled={busy}>
              {busy && <Loader2 aria-hidden className="animate-spin" />}
              {t("install.uninstall")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}

function ProgressList({ items }: { items: ItemProgress[] }) {
  const { t, i18n } = useTranslation();
  const size = (n: number) => formatBytes(n, i18n.language);
  const label = (c: string) => (i18n.exists(`install.component.${c}`) ? t(`install.component.${c}` as never) : c);
  return (
    <ul className="space-y-2" aria-label={t("install.install")} data-testid="install-progress">
      {items.map((i) => {
        const pct =
          i.phase === "installed"
            ? 100
            : i.phase === "downloading" && i.total > 0
              ? Math.round((i.done / i.total) * 100)
              : i.phase === "queued"
                ? 0
                : i.phase === "downloading"
                  ? 0
                  : 95;
        const text =
          i.phase === "downloading"
            ? t("install.phase.downloading", { done: size(i.done), total: size(i.total) })
            : t(`install.phase.${i.phase}`);
        return (
          <li key={i.component} className="space-y-1" data-testid={`progress-${i.component}`} data-phase={i.phase}>
            <div className="flex items-center justify-between gap-3 text-xs">
              <span className="font-medium">{label(i.component)}</span>
              <span className="inline-flex items-center gap-1 text-muted-foreground">
                {i.phase === "installed" ? (
                  <CheckCircle2 aria-hidden className="size-3.5 text-verdict-accepted" />
                ) : i.phase === "queued" ? null : (
                  <Loader2 aria-hidden className="size-3.5 animate-spin" />
                )}
                {text}
              </span>
            </div>
            <Progress value={pct} aria-label={label(i.component)} />
          </li>
        );
      })}
    </ul>
  );
}
