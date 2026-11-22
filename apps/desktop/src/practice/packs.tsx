// Problem packs (expansion M11, ADR 0038) on Settings → Packs: import (a native dialog
// opened by Rust, then a preview with what the pack is and does: T13), the installed
// packs, uninstall, the practice option (review A3), and the restart that loads changes.
import { useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { Package, RotateCw, ShieldAlert, Trash2, Upload } from "lucide-react";
import type { PackPreview } from "@/bindings";
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
import { ErrorNotice } from "@/components/ErrorNotice";
import { api } from "@/lib/api";
import { useSetting } from "@/lib/settings";

const key = ["packs"] as const;

export function PacksSection() {
  const { t, i18n } = useTranslation();
  const qc = useQueryClient();
  const packs = useQuery({ queryKey: key, queryFn: () => api.packs() });
  const [preview, setPreview] = useState<PackPreview | null>(null);
  const [restart, setRestart] = useState(false);
  const [includePacks, setIncludePacks] = useSetting<boolean>("practice.include_packs", false, (v) =>
    typeof v === "boolean" ? v : undefined,
  );
  const open = useMutation({ mutationFn: () => api.importPackPreview(), onSuccess: (p) => p && setPreview(p) });
  const apply = useMutation({
    mutationFn: (token: number) => api.importPackApply(token),
    onSuccess: () => {
      setPreview(null);
      setRestart(true);
      void qc.invalidateQueries({ queryKey: key });
    },
  });
  const remove = useMutation({
    mutationFn: (id: string) => api.uninstallPack(id),
    onSuccess: () => {
      setRestart(true);
      void qc.invalidateQueries({ queryKey: key });
    },
  });
  const close = () => {
    if (preview) void api.importPackCancel(preview.token);
    setPreview(null);
    apply.reset();
  };
  const fmt = new Intl.DateTimeFormat(i18n.language, { dateStyle: "medium" });
  const canApply = preview && (preview.action === "install" || preview.action === "update");
  return (
    <div className="space-y-3" data-testid="packs">
      <p className="text-sm text-muted-foreground">{t("packs.intro")}</p>
      <div className="flex flex-wrap items-center gap-2">
        <Button size="sm" variant="outline" onClick={() => open.mutate()} disabled={open.isPending} data-testid="pack-import">
          <Upload aria-hidden />
          {t("packs.import")}
        </Button>
        <label className="flex items-center gap-2 text-sm">
          <Checkbox
            checked={includePacks}
            onCheckedChange={(v) => setIncludePacks(v === true)}
            aria-label={t("packs.includeInPractice")}
          />
          {t("packs.includeInPractice")}
        </label>
      </div>
      {open.error && <ErrorNotice error={open.error} />}
      {remove.error && <ErrorNotice error={remove.error} />}
      {restart && (
        <p role="status" className="flex flex-wrap items-center gap-2 rounded-md border bg-muted/40 p-2 text-sm" data-testid="pack-restart">
          {t("packs.restartNeeded")}
          <Button size="sm" onClick={() => void api.restartApp()}>
            <RotateCw aria-hidden />
            {t("packs.restart")}
          </Button>
        </p>
      )}
      {(packs.data?.length ?? 0) === 0 ? (
        <p className="text-sm text-muted-foreground">{t("packs.none")}</p>
      ) : (
        <ul className="divide-y rounded-lg border" data-testid="pack-list">
          {packs.data!.map((p) => (
            <li key={p.id} className="flex flex-wrap items-center gap-x-3 gap-y-1 px-3 py-2 text-sm">
              <Package aria-hidden className="size-4 text-muted-foreground" />
              <span className="font-medium">{p.name}</span>
              <span className="text-muted-foreground">
                {t("packs.row", { version: p.version, author: p.author, count: p.problems, date: fmt.format(new Date(p.installed_at)) })}
              </span>
              <Button
                size="sm"
                variant="ghost"
                className="ml-auto"
                onClick={() => remove.mutate(p.id)}
                aria-label={t("packs.uninstall", { name: p.name })}
              >
                <Trash2 aria-hidden />
              </Button>
            </li>
          ))}
        </ul>
      )}
      <Dialog open={preview !== null} onOpenChange={(o) => !o && close()}>
        <DialogContent data-testid="pack-preview">
          {preview && (
            <>
              <DialogHeader>
                <DialogTitle>{t("packs.previewTitle", { name: preview.name })}</DialogTitle>
                <DialogDescription className="flex items-start gap-1.5">
                  <ShieldAlert aria-hidden className="mt-0.5 size-4 flex-none" />
                  {t("packs.unsigned", { author: preview.author })}
                </DialogDescription>
              </DialogHeader>
              <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-sm">
                <dt className="text-muted-foreground">{t("packs.id")}</dt>
                <dd className="font-mono">{preview.id}</dd>
                <dt className="text-muted-foreground">{t("packs.version")}</dt>
                <dd>
                  {preview.version}
                  {preview.installed && ` (${t("packs.installedVersion", { version: preview.installed })})`}
                </dd>
                <dt className="text-muted-foreground">{t("packs.licence")}</dt>
                <dd>{preview.licence}</dd>
                <dt className="text-muted-foreground">{t("packs.problems")}</dt>
                <dd>{preview.problems}</dd>
                <dt className="text-muted-foreground">SHA-256</dt>
                <dd className="font-mono text-xs break-all" data-testid="pack-sha">
                  {preview.sha256}
                </dd>
              </dl>
              <p className="text-sm" data-testid="pack-action">
                {t(`packs.action.${preview.action}`)}
              </p>
              {apply.error && <ErrorNotice error={apply.error} />}
              <DialogFooter>
                <Button variant="outline" onClick={close}>
                  {t("packs.cancel")}
                </Button>
                {canApply && (
                  <Button onClick={() => apply.mutate(preview.token)} disabled={apply.isPending} data-testid="pack-apply">
                    {t(preview.action === "update" ? "packs.update" : "packs.install")}
                  </Button>
                )}
              </DialogFooter>
            </>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
