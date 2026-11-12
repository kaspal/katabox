import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Download, Gauge, Loader2, Monitor, Moon, Sun, Upload } from "lucide-react";
import type { ImportPreview, TransferCounts } from "@/bindings";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Page } from "@/components/Page";
import { InstallPanel } from "@/languages/InstallPanel";
import { ErrorNotice } from "@/components/ErrorNotice";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { MiddleEllipsis } from "@/components/MiddleEllipsis";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { UI_LANGS, type UiLangCode } from "@/i18n";
import { THEMES, usePrefs, type ThemePref } from "@/lib/prefs";
import { api } from "@/lib/api";
import { queryKeys, useAppInfo, useCalibration } from "@/lib/queries";
import { usePracticePrefs } from "@/lib/practice";
import { FONT_SIZES, KEYMAPS, useEditorPrefs, type KeymapPref } from "@/lib/settings";

const THEME_ICON: Record<ThemePref, typeof Sun> = { system: Monitor, light: Sun, dark: Moon };

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="space-y-3 rounded-lg border bg-card p-5">
      <h2 className="text-sm font-semibold text-muted-foreground">{title}</h2>
      {children}
    </section>
  );
}

function Row({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div className="grid grid-cols-[180px_1fr] items-center gap-4 text-sm">
      <dt className="text-muted-foreground">{label}</dt>
      <dd className="min-w-0 break-all">{children}</dd>
    </div>
  );
}

/** Settings → Data (ADR 0020): export and import progress. The file dialogs open in
 * Rust; the page never sees a path. */
function DataSection() {
  const { t, i18n } = useTranslation();
  const queryClient = useQueryClient();
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [error, setError] = useState<unknown>(null);
  const [preview, setPreview] = useState<ImportPreview | null>(null);
  const run = async (job: () => Promise<void>) => {
    setBusy(true);
    setNotice(null);
    setError(null);
    try {
      await job();
    } catch (e) {
      setError(e);
    } finally {
      setBusy(false);
    }
  };
  const total = (c: TransferCounts) => Object.values(c).reduce((a, b) => a + b, 0);
  return (
    <Section title={t("practice.data.title")}>
      <p className="text-sm text-muted-foreground">{t("practice.data.intro")}</p>
      <div className="flex gap-2">
        <Button
          variant="outline"
          size="sm"
          disabled={busy}
          onClick={() =>
            void run(async () => {
              const c = await api.exportProgress();
              if (c) setNotice(t("practice.data.exported", { count: total(c) }));
            })
          }
        >
          <Download aria-hidden />
          {t("practice.data.export")}
        </Button>
        <Button
          variant="outline"
          size="sm"
          disabled={busy}
          onClick={() =>
            void run(async () => {
              const p = await api.importPreview();
              if (p) setPreview(p);
            })
          }
        >
          <Upload aria-hidden />
          {t("practice.data.import")}
        </Button>
      </div>
      {notice && (
        <p role="status" className="text-sm" data-testid="data-notice">
          {notice}
        </p>
      )}
      {error !== null && <ErrorNotice error={error} />}
      <Dialog
        open={preview !== null}
        onOpenChange={(open) => {
          if (!open && preview) {
            void api.importCancel(preview.id);
            setPreview(null);
          }
        }}
      >
        <DialogContent className="sm:max-w-md" data-testid="import-preview">
          <DialogHeader>
            <DialogTitle>{t("practice.data.previewTitle")}</DialogTitle>
            {preview && (
              <DialogDescription>
                {t("practice.data.previewFrom", {
                  date: new Intl.DateTimeFormat(i18n.language, { dateStyle: "medium" }).format(
                    new Date(preview.exported_at),
                  ),
                  version: preview.app_version,
                })}
              </DialogDescription>
            )}
          </DialogHeader>
          {preview && (
            <>
              <dl className="grid grid-cols-[1fr_auto] gap-x-6 gap-y-1 text-sm">
                {(Object.entries(preview.counts) as [keyof TransferCounts, number][])
                  .filter(([, n]) => n > 0)
                  .map(([kind, n]) => (
                    <div key={kind} className="contents">
                      <dt className="text-muted-foreground">{t(`practice.data.kinds.${kind}`)}</dt>
                      <dd className="text-right tabular-nums">{n}</dd>
                    </div>
                  ))}
              </dl>
              {preview.conflicts > 0 && (
                <p className="text-sm">{t("practice.data.conflicts", { count: preview.conflicts })}</p>
              )}
            </>
          )}
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => {
                if (preview) void api.importCancel(preview.id);
                setPreview(null);
              }}
            >
              {t("practice.data.cancel")}
            </Button>
            <Button
              disabled={busy}
              onClick={() =>
                void run(async () => {
                  if (!preview) return;
                  const r = await api.importApply(preview.id);
                  setPreview(null);
                  setNotice(t("practice.data.imported", r));
                  await queryClient.invalidateQueries();
                })
              }
            >
              {t("practice.data.apply")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Section>
  );
}

/** Settings → Practice (ADR 0019): the Home cards for reviews and the daily problem. */
function PracticeSection() {
  const { t } = useTranslation();
  const prefs = usePracticePrefs();
  const toggle = (label: string, value: boolean, set: (v: boolean) => void) => (
    <Row label={label}>
      <ToggleGroup
        type="single"
        variant="outline"
        size="sm"
        aria-label={label}
        value={value ? "on" : "off"}
        onValueChange={(v) => v && set(v === "on")}
      >
        <ToggleGroupItem value="on" className="px-3">
          {t("practice.settings.on")}
        </ToggleGroupItem>
        <ToggleGroupItem value="off" className="px-3">
          {t("practice.settings.off")}
        </ToggleGroupItem>
      </ToggleGroup>
    </Row>
  );
  return (
    <Section title={t("practice.settings.title")}>
      <dl className="space-y-3">
        {toggle(t("practice.settings.daily"), prefs.daily, prefs.setDaily)}
        {toggle(t("practice.settings.review"), prefs.review, prefs.setReview)}
      </dl>
    </Section>
  );
}

/** The judge's speed factor for this machine (ADR 0017), with Recalibrate. */
function CalibrationSection() {
  const { t, i18n } = useTranslation();
  const queryClient = useQueryClient();
  const calibration = useCalibration();
  const recalibrate = useMutation({
    mutationFn: api.recalibrate,
    onSuccess: (c) => {
      queryClient.setQueryData(queryKeys.calibration, c);
      // Every problem's limits follow the new factor.
      void queryClient.invalidateQueries({ queryKey: ["problem"] });
    },
  });
  const c = calibration.data;
  const factor = (x: number) => t("settings.calibration.factorValue", { factor: x.toFixed(2) });
  return (
    <Section title={t("settings.calibration.title")}>
      <p className="text-sm text-muted-foreground">{t("settings.calibration.description")}</p>
      {calibration.isPending ? (
        <Skeleton className="h-16 w-full" />
      ) : calibration.error ? (
        <ErrorNotice error={calibration.error} onRetry={() => void calibration.refetch()} />
      ) : c ? (
        <dl className="space-y-2" data-testid="calibration">
          <Row label={t("settings.calibration.factor")}>
            <span className="font-mono">{factor(c.applied)}</span>
            {Math.abs(c.factor - c.applied) >= 0.005 && (
              <span className="ml-2 text-muted-foreground">
                {t("settings.calibration.clamped", { measured: factor(c.factor) })}
              </span>
            )}
          </Row>
          <Row label={t("settings.calibration.runtime")}>{t(`languages.${c.runtime}`)}</Row>
          <Row label={t("settings.calibration.measuredAt")}>
            {new Intl.DateTimeFormat(i18n.language, { dateStyle: "medium", timeStyle: "short" }).format(
              new Date(c.measured_at * 1000),
            )}
          </Row>
        </dl>
      ) : (
        <p className="text-sm">{t("settings.calibration.notMeasured")}</p>
      )}
      {recalibrate.error && <ErrorNotice error={recalibrate.error} />}
      <Button
        size="sm"
        variant="outline"
        disabled={recalibrate.isPending}
        onClick={() => recalibrate.mutate()}
      >
        {recalibrate.isPending ? <Loader2 className="animate-spin" aria-hidden /> : <Gauge aria-hidden />}
        {recalibrate.isPending ? t("settings.calibration.measuring") : t("settings.calibration.recalibrate")}
      </Button>
    </Section>
  );
}

export function Settings() {
  const { t } = useTranslation();
  const { theme, lang, setTheme, setLang } = usePrefs();
  const info = useAppInfo();
  const editor = useEditorPrefs();

  return (
    <Page title={t("settings.title")}>
      <div className="max-w-3xl space-y-4">
        <Section title={t("settings.appearance")}>
          <dl className="space-y-3">
            <Row label={t("theme.label")}>
              <ToggleGroup
                type="single"
                variant="outline"
                size="sm"
                aria-label={t("theme.label")}
                value={theme}
                onValueChange={(v) => v && setTheme(v as ThemePref)}
              >
                {THEMES.map((th) => {
                  const Icon = THEME_ICON[th];
                  return (
                    <ToggleGroupItem key={th} value={th} className="gap-1.5 px-3">
                      <Icon aria-hidden />
                      {t(`theme.${th}`)}
                    </ToggleGroupItem>
                  );
                })}
              </ToggleGroup>
            </Row>
            <Row label={t("language.label")}>
              <ToggleGroup
                type="single"
                variant="outline"
                size="sm"
                aria-label={t("language.label")}
                value={lang}
                onValueChange={(v) => v && setLang(v as UiLangCode)}
              >
                {UI_LANGS.map((l) => (
                  <ToggleGroupItem key={l} value={l} lang={l} className="px-3">
                    {t(`language.${l}`)}
                  </ToggleGroupItem>
                ))}
              </ToggleGroup>
            </Row>
          </dl>
        </Section>

        <Section title={t("settings.editor")}>
          <dl className="space-y-3">
            <Row label={t("settings.keymap")}>
              <ToggleGroup
                type="single"
                variant="outline"
                size="sm"
                aria-label={t("settings.keymap")}
                value={editor.keymap}
                onValueChange={(v) => v && editor.setKeymap(v as KeymapPref)}
              >
                {KEYMAPS.map((k) => (
                  <ToggleGroupItem key={k} value={k} className="px-3">
                    {t(`settings.keymaps.${k}`)}
                  </ToggleGroupItem>
                ))}
              </ToggleGroup>
            </Row>
            <Row label={t("settings.ligatures")}>
              <ToggleGroup
                type="single"
                variant="outline"
                size="sm"
                aria-label={t("settings.ligatures")}
                value={editor.ligatures ? "on" : "off"}
                onValueChange={(v) => v && editor.setLigatures(v === "on")}
              >
                <ToggleGroupItem value="off" className="px-3">
                  {t("settings.ligaturesOff")}
                </ToggleGroupItem>
                <ToggleGroupItem value="on" className="px-3">
                  {t("settings.ligaturesOn")}
                </ToggleGroupItem>
              </ToggleGroup>
            </Row>
            <Row label={t("settings.fontSize")}>
              <Select value={String(editor.fontSize)} onValueChange={(v) => editor.setFontSize(Number(v))}>
                <SelectTrigger size="sm" className="w-28" aria-label={t("settings.fontSize")}>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {FONT_SIZES.map((n) => (
                    <SelectItem key={n} value={String(n)}>
                      {n} px
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </Row>
          </dl>
        </Section>

        <PracticeSection />

        <DataSection />

        <Section title={t("settings.languages")}>
          <InstallPanel allowUninstall />
        </Section>

        <CalibrationSection />

        <Section title={t("settings.about")}>
          {info.isPending ? (
            <Skeleton className="h-32 w-full" />
          ) : info.error ? (
            <ErrorNotice error={info.error} onRetry={() => void info.refetch()} />
          ) : (
            <dl className="space-y-2">
              <Row label={t("settings.version")}>
                <span className="font-mono">{info.data.app_version}</span>
                {info.data.dev_data && (
                  <Badge className="ml-2 bg-dev-badge font-mono text-dev-badge-foreground">
                    {t("app.devData")}
                  </Badge>
                )}
              </Row>
              <Row label={t("settings.contentPack")}>
                <span className="font-mono">
                  {info.data.content.pack_id} {info.data.content.pack_version}
                </span>
              </Row>
              <Row label={t("settings.userSchema")}>
                <span className="font-mono">{info.data.user_schema_version}</span>
              </Row>
              <Row label={t("settings.dataFolder")}>
                <MiddleEllipsis className="font-mono text-xs" text={info.data.data_dir} />
              </Row>
              <Row label={t("settings.sandboxRoot")}>
                <MiddleEllipsis className="font-mono text-xs" text={info.data.sandbox_root} />
              </Row>
              <Row label={t("settings.sandboxSource")}>
                {t(`settings.rootSource.${info.data.sandbox_root_source}`)}
              </Row>
            </dl>
          )}
        </Section>
      </div>
    </Page>
  );
}
