// Automatic backups (expansion M10, ADR 0037), on Settings → Data: the folder (chosen in a
// native dialog opened by Rust), how often, how many to keep, on exit, Back up now, and
// the status. Backups are ordinary exports; KataBox deletes only its own backup files.
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { FolderOpen, HardDriveDownload, TriangleAlert, X } from "lucide-react";
import type { BackupStatus } from "@/bindings";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { ErrorNotice } from "@/components/ErrorNotice";
import { api } from "@/lib/api";

const key = ["backup_status"] as const;

export function BackupSection() {
  const { t, i18n } = useTranslation();
  const qc = useQueryClient();
  const status = useQuery({ queryKey: key, queryFn: () => api.backupStatus() });
  const set = (s: BackupStatus | null) => s && qc.setQueryData(key, s);
  const choose = useMutation({ mutationFn: () => api.chooseBackupFolder(), onSuccess: set });
  const clear = useMutation({ mutationFn: () => api.clearBackupFolder(), onSuccess: set });
  const now = useMutation({
    mutationFn: () => api.backupNow(),
    onSuccess: set,
    onError: () => void qc.invalidateQueries({ queryKey: key }),
  });
  const options = useMutation({
    mutationFn: (o: { every: number; keep: number; onExit: boolean }) => api.setBackupOptions(o.every, o.keep, o.onExit),
    onSuccess: set,
  });
  const s = status.data;
  if (!s) return null;
  const fmt = new Intl.DateTimeFormat(i18n.language, { dateStyle: "medium", timeStyle: "short" });
  const save = (patch: Partial<{ every: number; keep: number; onExit: boolean }>) =>
    options.mutate({ every: s.every_days, keep: s.keep, onExit: s.on_exit, ...patch });
  return (
    <div className="space-y-3" data-testid="backups">
      <p className="text-sm text-muted-foreground">{t("backup.intro")}</p>
      <div className="flex flex-wrap items-center gap-2">
        <span className="min-w-0 truncate rounded-md border bg-muted/40 px-2 py-1 font-mono text-xs" data-testid="backup-folder">
          {s.folder ?? t("backup.noFolder")}
        </span>
        <Button size="sm" variant="outline" onClick={() => choose.mutate()} disabled={choose.isPending}>
          <FolderOpen aria-hidden />
          {t(s.folder ? "backup.change" : "backup.choose")}
        </Button>
        {s.folder && (
          <Button size="sm" variant="ghost" onClick={() => clear.mutate()}>
            <X aria-hidden />
            {t("backup.off")}
          </Button>
        )}
      </div>
      {s.folder && (
        <>
          <div className="flex flex-wrap items-end gap-4">
            <label className="space-y-1 text-xs">
              <span className="text-muted-foreground">{t("backup.every")}</span>
              <Input
                type="number"
                min={1}
                max={90}
                defaultValue={s.every_days}
                onBlur={(e) => {
                  const v = Number.parseInt(e.target.value, 10);
                  if (v >= 1 && v <= 90 && v !== s.every_days) save({ every: v });
                }}
                className="h-8 w-20"
              />
            </label>
            <label className="space-y-1 text-xs">
              <span className="text-muted-foreground">{t("backup.keep")}</span>
              <Input
                type="number"
                min={1}
                max={100}
                defaultValue={s.keep}
                onBlur={(e) => {
                  const v = Number.parseInt(e.target.value, 10);
                  if (v >= 1 && v <= 100 && v !== s.keep) save({ keep: v });
                }}
                className="h-8 w-20"
              />
            </label>
            <label className="flex items-center gap-2 pb-1.5 text-sm">
              <Checkbox checked={s.on_exit} onCheckedChange={(v) => save({ onExit: v === true })} />
              {t("backup.onExit")}
            </label>
          </div>
          <div className="flex flex-wrap items-center gap-3 text-sm">
            <Button size="sm" onClick={() => now.mutate()} disabled={now.isPending} data-testid="backup-now">
              <HardDriveDownload aria-hidden />
              {t("backup.now")}
            </Button>
            <span className="text-muted-foreground" data-testid="backup-status">
              {s.last ? t("backup.last", { when: fmt.format(new Date(s.last)) }) : t("backup.never")}
              {s.next_due !== null && ` · ${t("backup.next", { when: fmt.format(new Date(s.next_due)) })}`}
            </span>
          </div>
          {s.error && (
            <p role="status" className="flex items-start gap-1.5 text-sm text-verdict-wrong" data-testid="backup-error">
              <TriangleAlert aria-hidden className="mt-0.5 size-4 flex-none" />
              {t("backup.failed", { why: s.error })}
            </p>
          )}
          <p className="text-xs text-muted-foreground">{t("backup.prune")}</p>
        </>
      )}
      {(choose.error || now.error || options.error) && <ErrorNotice error={choose.error ?? now.error ?? options.error} />}
    </div>
  );
}
