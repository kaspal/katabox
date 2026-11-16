// Settings (ui-polish): sections behind a labelled side-nav (stacked tabs on narrow
// windows), each linkable as `/settings?section=<id>` and the last one remembered
// (`settings.section`). A search box finds a setting by its label and opens its section.
import { Suspense, lazy, useCallback, useMemo, useState } from "react";
import { useNavigate, useSearch } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { AppWindow, Download, Gauge, Loader2, Monitor, Moon, Search, Sun, Upload } from "lucide-react";
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
import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { normalizeForSearch } from "@/app/paletteFilter";
import { useLearnSpeed } from "@/learn/engine/progress";
import { SPEEDS, type Speed } from "@/learn/engine/usePlayer";
import { useMedia } from "@/learn/engine/media";
import { SETTINGS_SECTIONS, useRememberedTab, type SettingsSection } from "@/lib/pageTabs";
import { Button } from "@/components/ui/button";
import { MiddleEllipsis } from "@/components/MiddleEllipsis";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { UI_LANGS, type UiLangCode } from "@/i18n";
import { CONTRASTS, MOTIONS, THEMES, usePrefs, type ContrastPref, type MotionPref, type ThemePref } from "@/lib/prefs";
import { api } from "@/lib/api";
import { useOnboardingDone } from "@/onboarding/state";
import { copyText, diagnosticsText, recentErrors } from "@/lib/diagnostics";
import { CHANGELOG_URL, DOCS_URL } from "@/lib/links";
import { openShortcuts } from "@/app/shortcuts";
import { queryKeys, useAppInfo, useCalibration } from "@/lib/queries";
import { usePracticePrefs } from "@/lib/practice";
import {
  EDITOR_THEMES,
  FONT_SIZES,
  KEYMAPS,
  TAB_WIDTHS,
  useEditorPrefs,
  type EditorTheme,
  type KeymapPref,
} from "@/lib/settings";

// CodeMirror lives in the lazy workspace chunk; the preview loads it only here.
const EditorPreview = lazy(() => import("@/workspace/editor/EditorPreview"));

const THEME_ICON: Record<ThemePref, typeof Sun> = { system: Monitor, light: Sun, dark: Moon };

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="space-y-3 rounded-lg border bg-card p-5">
      <h2 className="text-sm font-semibold text-muted-foreground">{title}</h2>
      {children}
    </section>
  );
}

/** Copy diagnostics: shows exactly what will be copied (versions, flags and recent
 * error codes; no code, paths or personal data), then copies it. */
function DiagnosticsButton() {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);
  const [copied, setCopied] = useState<"idle" | "done" | "failed">("idle");
  const diag = useQuery({
    queryKey: ["diagnostics"],
    queryFn: async () => diagnosticsText(await api.diagnostics(), recentErrors()),
    enabled: open,
    staleTime: 0,
    gcTime: 0,
  });
  return (
    <>
      <Button
        variant="outline"
        size="sm"
        onClick={() => {
          setCopied("idle");
          setOpen(true);
        }}
      >
        {t("settings.diagnostics.button")}
      </Button>
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="sm:max-w-2xl" data-testid="diagnostics">
          <DialogHeader>
            <DialogTitle>{t("settings.diagnostics.title")}</DialogTitle>
            <DialogDescription>{t("settings.diagnostics.intro")}</DialogDescription>
          </DialogHeader>
          {diag.isPending ? (
            <Skeleton className="h-48 w-full" />
          ) : diag.error ? (
            <ErrorNotice error={diag.error} onRetry={() => void diag.refetch()} />
          ) : (
            <pre
              className="max-h-[50vh] overflow-auto whitespace-pre-wrap rounded-md border bg-muted p-3 font-mono text-xs"
              data-testid="diagnostics-text"
            >
              {diag.data}
            </pre>
          )}
          <div className="flex items-center justify-end gap-3">
            <span role="status" className="text-sm text-muted-foreground">
              {copied === "done"
                ? t("settings.diagnostics.copied")
                : copied === "failed"
                  ? t("settings.diagnostics.copyFailed")
                  : ""}
            </span>
            <Button
              size="sm"
              disabled={!diag.data}
              onClick={async () => setCopied((await copyText(diag.data ?? "")) ? "done" : "failed")}
            >
              {t("settings.diagnostics.copy")}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}

/** An address shown as selectable text with a Copy button (About opens no browser). */
function CopyableUrl({ url, testId }: { url: string; testId: string }) {
  const { t } = useTranslation();
  const [copied, setCopied] = useState<"idle" | "done" | "failed">("idle");
  return (
    <span className="flex flex-wrap items-center gap-2">
      <span className="select-all break-all font-mono text-xs" data-testid={testId}>
        {url}
      </span>
      <Button variant="outline" size="sm" onClick={async () => setCopied((await copyText(url)) ? "done" : "failed")}>
        {t("settings.copyAddress")}
      </Button>
      <span role="status" className="text-xs text-muted-foreground">
        {copied === "done" ? t("settings.diagnostics.copied") : copied === "failed" ? t("settings.diagnostics.copyFailed") : ""}
      </span>
    </span>
  );
}

/** Settings → About → Third-party notices: the bundled THIRD_PARTY_NOTICES.txt, read by
 * Rust (ADR 0021). Development builds have none. */
function NoticesButton() {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);
  const notices = useQuery({
    queryKey: ["third-party-notices"],
    queryFn: () => api.thirdPartyNotices(),
    enabled: open,
    staleTime: Infinity,
  });
  return (
    <>
      <Button variant="outline" size="sm" onClick={() => setOpen(true)}>
        {t("settings.noticesView")}
      </Button>
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="sm:max-w-3xl" data-testid="notices">
          <DialogHeader>
            <DialogTitle>{t("settings.notices")}</DialogTitle>
            <DialogDescription>{t("settings.noticesIntro")}</DialogDescription>
          </DialogHeader>
          {notices.isPending ? (
            <Skeleton className="h-64 w-full" />
          ) : notices.error ? (
            <ErrorNotice error={notices.error} onRetry={() => void notices.refetch()} />
          ) : notices.data === null ? (
            <p className="text-sm text-muted-foreground" data-testid="notices-dev">
              {t("settings.noticesDev")}
            </p>
          ) : (
            <pre
              className="max-h-[60vh] overflow-auto whitespace-pre-wrap rounded-md border bg-muted p-3 font-mono text-xs"
              tabIndex={0}
              data-testid="notices-text"
            >
              {notices.data}
            </pre>
          )}
          <DialogFooter>
            <Button variant="outline" onClick={() => setOpen(false)}>
              {t("settings.close")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
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
    <Section title={t("settings.sections.data")}>
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
  const onboarding = useOnboardingDone();
  const navigate = useNavigate();
  return (
    <Section title={t("settings.sections.practice")}>
      <dl className="space-y-3">
        <OnOffRow label={t("practice.settings.daily")} value={prefs.daily} set={prefs.setDaily} />
        <OnOffRow label={t("practice.settings.review")} value={prefs.review} set={prefs.setReview} />
        <Row label={t("welcome.again.label")}>
          <Button
            variant="outline"
            size="sm"
            onClick={() => {
              onboarding.setDone(false);
              void navigate({ to: "/" });
            }}
          >
            {t("welcome.again.button")}
          </Button>
        </Row>
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

/** An On/Off switch row (the Home cards). */
function OnOffRow({ label, value, set }: { label: string; value: boolean; set: (v: boolean) => void }) {
  const { t } = useTranslation();
  return (
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
}

/** Settings → Learn: the Home card for Learn, and the animations' playback speed. */
function LearnSection() {
  const { t } = useTranslation();
  const prefs = usePracticePrefs();
  const [speed, setSpeed] = useLearnSpeed();
  return (
    <Section title={t("settings.sections.learn")}>
      <dl className="space-y-3">
        <OnOffRow label={t("practice.settings.learnReview")} value={prefs.learnReview} set={prefs.setLearnReview} />
        <Row label={t("settings.learnSpeed")}>
          <ToggleGroup
            type="single"
            variant="outline"
            size="sm"
            aria-label={t("settings.learnSpeed")}
            value={String(speed)}
            onValueChange={(v) => v && setSpeed(Number(v) as Speed)}
          >
            {SPEEDS.map((n) => (
              <ToggleGroupItem key={n} value={String(n)} className="px-3 tabular-nums">
                {`${n}×`}
              </ToggleGroupItem>
            ))}
          </ToggleGroup>
        </Row>
      </dl>
    </Section>
  );
}

/** Settings → Appearance → Window: back to the default size, centred (window memory). */
function ResetWindowRow() {
  const { t } = useTranslation();
  const reset = useMutation({ mutationFn: api.resetWindow });
  return (
    <Row label={t("settings.window.label")}>
      <span className="flex flex-wrap items-center gap-3">
        <Button variant="outline" size="sm" disabled={reset.isPending} onClick={() => reset.mutate()}>
          <AppWindow aria-hidden />
          {t("settings.window.reset")}
        </Button>
        <span role="status" className="text-sm text-muted-foreground">
          {reset.isSuccess ? t("settings.window.done") : ""}
        </span>
      </span>
      {reset.error && <ErrorNotice error={reset.error} />}
    </Row>
  );
}

/** What "Search settings" finds: each section's row labels (i18n keys). */
const SEARCH_INDEX: [SettingsSection, string[]][] = [
  [
    "appearance",
    ["theme.label", "language.label", "settings.contrast", "settings.motion", "settings.window.label", "settings.window.reset"],
  ],
  [
    "editor",
    ["settings.keymap", "settings.ligatures", "settings.fontSize", "settings.tabWidth", "settings.editorTheme", "settings.preview"],
  ],
  ["practice", ["practice.settings.daily", "practice.settings.review", "welcome.again.label"]],
  ["languages", ["settings.sections.languages", "settings.calibration.title", "settings.calibration.recalibrate"]],
  ["learn", ["practice.settings.learnReview", "settings.learnSpeed"]],
  ["data", ["practice.data.export", "practice.data.import"]],
  [
    "about",
    [
      "settings.version",
      "settings.contentPack",
      "settings.licenses.label",
      "settings.userSchema",
      "settings.dataFolder",
      "settings.sandboxRoot",
      "settings.notices",
      "settings.docs",
      "settings.changelog",
      "shortcuts.title",
      "settings.diagnostics.label",
    ],
  ],
];

/** Finds a setting by its label (accents and punctuation ignored) and opens its section. */
function SettingsSearch({ onPick }: { onPick: (section: SettingsSection) => void }) {
  const { t } = useTranslation();
  const [query, setQuery] = useState("");
  const results = useMemo(() => {
    const q = normalizeForSearch(query);
    if (q === "") return [];
    const label = t as unknown as (key: string) => string;
    return SEARCH_INDEX.flatMap(([section, keys]) =>
      keys
        .map((k) => label(k))
        .filter((text) => normalizeForSearch(text).includes(q))
        .map((text) => ({ section, text })),
    );
  }, [query, t]);
  return (
    <div className="space-y-2">
      <div className="relative">
        <Search
          aria-hidden
          className="pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground"
        />
        <Input
          type="search"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          aria-label={t("settings.search.label")}
          placeholder={t("settings.search.placeholder")}
          className="h-8 pl-8"
          data-testid="settings-search"
        />
      </div>
      {query.trim() !== "" &&
        (results.length === 0 ? (
          <p className="px-1 text-sm text-muted-foreground" role="status">
            {t("settings.search.none")}
          </p>
        ) : (
          <ul className="space-y-0.5" aria-label={t("settings.search.results")} data-testid="settings-search-results">
            {results.map((r) => (
              <li key={`${r.section}:${r.text}`}>
                <button
                  type="button"
                  className="w-full rounded-md px-2 py-1 text-left text-sm hover:bg-accent focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-none"
                  onClick={() => {
                    setQuery("");
                    onPick(r.section);
                  }}
                >
                  <span className="block">{r.text}</span>
                  <span className="block text-xs text-muted-foreground">{t(`settings.sections.${r.section}`)}</span>
                </button>
              </li>
            ))}
          </ul>
        ))}
    </div>
  );
}

export function Settings() {
  const { t } = useTranslation();
  const { theme, lang, contrast, motion, setTheme, setLang, setContrast, setMotion } = usePrefs();
  const info = useAppInfo();
  const editor = useEditorPrefs();
  const navigate = useNavigate({ from: "/settings" });
  const search = useSearch({ from: "/settings" });
  const toUrl = useCallback(
    (section: SettingsSection) => void navigate({ search: { section }, replace: true }),
    [navigate],
  );
  const [section, setSection] = useRememberedTab(
    "settings.section",
    SETTINGS_SECTIONS,
    "appearance",
    search.section,
    toUrl,
  );
  // A side-nav when there is room; stacked tabs above the section otherwise.
  const wide = useMedia("(min-width: 1100px)");

  return (
    <Page title={t("settings.title")}>
      <Tabs
        value={section}
        onValueChange={setSection}
        orientation={wide ? "vertical" : "horizontal"}
        className="min-h-0 max-w-5xl flex-1 items-start gap-6"
      >
        <nav aria-label={t("settings.sections.label")} className={wide ? "w-56 shrink-0 space-y-3" : "w-full space-y-2"}>
          <SettingsSearch onPick={setSection} />
          <TabsList
            variant="line"
            data-testid="settings-sections"
            className={wide ? "w-full items-stretch" : "h-auto w-full flex-wrap justify-start"}
          >
            {SETTINGS_SECTIONS.map((id) => (
              <TabsTrigger
                key={id}
                value={id}
                className={wide ? "justify-start px-3 py-1.5" : "flex-none px-3"}
                data-testid={`settings-section-${id}`}
              >
                {t(`settings.sections.${id}`)}
              </TabsTrigger>
            ))}
          </TabsList>
        </nav>

        <div className="w-full min-w-0 max-w-3xl">
          <TabsContent value="appearance">
            <Section title={t("settings.sections.appearance")}>
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
                <Row label={t("settings.contrast")}>
                  <ToggleGroup
                    type="single"
                    variant="outline"
                    size="sm"
                    aria-label={t("settings.contrast")}
                    value={contrast}
                    onValueChange={(v) => v && setContrast(v as ContrastPref)}
                  >
                    {CONTRASTS.map((c) => (
                      <ToggleGroupItem key={c} value={c} className="px-3">
                        {t(`settings.contrasts.${c}`)}
                      </ToggleGroupItem>
                    ))}
                  </ToggleGroup>
                </Row>
                <Row label={t("settings.motion")}>
                  <ToggleGroup
                    type="single"
                    variant="outline"
                    size="sm"
                    aria-label={t("settings.motion")}
                    value={motion}
                    onValueChange={(v) => v && setMotion(v as MotionPref)}
                  >
                    {MOTIONS.map((m) => (
                      <ToggleGroupItem key={m} value={m} className="px-3">
                        {t(`settings.motions.${m}`)}
                      </ToggleGroupItem>
                    ))}
                  </ToggleGroup>
                </Row>
                <ResetWindowRow />
              </dl>
            </Section>
          </TabsContent>

          <TabsContent value="editor">
            <Section title={t("settings.sections.editor")}>
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
                <Row label={t("settings.tabWidth")}>
                  <Select value={String(editor.tabWidth)} onValueChange={(v) => editor.setTabWidth(Number(v))}>
                    <SelectTrigger size="sm" className="w-44" aria-label={t("settings.tabWidth")}>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {TAB_WIDTHS.map((n) => (
                        <SelectItem key={n} value={String(n)}>
                          {n === 0 ? t("settings.tabWidthLanguage") : t("settings.tabWidthSpaces", { count: n })}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </Row>
                <Row label={t("settings.editorTheme")}>
                  <Select value={editor.theme} onValueChange={(v) => editor.setTheme(v as EditorTheme)}>
                    <SelectTrigger size="sm" className="w-44" aria-label={t("settings.editorTheme")}>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {EDITOR_THEMES.map((id) => (
                        <SelectItem key={id} value={id}>
                          {t(`settings.editorThemes.${id}`)}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </Row>
              </dl>
              <div className="mt-4 space-y-1.5">
                <p className="text-xs font-medium text-muted-foreground">{t("settings.preview")}</p>
                <Suspense fallback={<Skeleton className="h-56 w-full" />}>
                  <EditorPreview />
                </Suspense>
              </div>
            </Section>
          </TabsContent>

          <TabsContent value="practice">
            <PracticeSection />
          </TabsContent>

          <TabsContent value="languages" className="space-y-4">
            <Section title={t("settings.sections.languages")}>
              <InstallPanel allowUninstall />
            </Section>
            <CalibrationSection />
          </TabsContent>

          <TabsContent value="learn">
            <LearnSection />
          </TabsContent>

          <TabsContent value="data">
            <DataSection />
          </TabsContent>

          <TabsContent value="about">
            <Section title={t("settings.sections.about")}>
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
                    <span className="font-mono" data-testid="content-pack">
                      {info.data.content.pack_id} {info.data.content.pack_version}
                    </span>
                    {info.data.content.pack_date && (
                      <span className="ml-2 text-muted-foreground">
                        {t("settings.contentDate", { date: info.data.content.pack_date })}
                      </span>
                    )}
                  </Row>
                  <Row label={t("settings.licenses.label")}>
                    <span className="block">{t("settings.licenses.code")}</span>
                    <span className="block">{t("settings.licenses.content")}</span>
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
                  <Row label={t("settings.notices")}>
                    <NoticesButton />
                  </Row>
                  <Row label={t("settings.docs")}>
                    <CopyableUrl url={DOCS_URL} testId="docs-url" />
                  </Row>
                  <Row label={t("settings.changelog")}>
                    <CopyableUrl url={CHANGELOG_URL} testId="changelog-url" />
                  </Row>
                  <Row label={t("shortcuts.title")}>
                    <Button variant="outline" size="sm" onClick={openShortcuts}>
                      {t("shortcuts.open")}
                    </Button>
                  </Row>
                  <Row label={t("settings.diagnostics.label")}>
                    <DiagnosticsButton />
                  </Row>
                </dl>
              )}
            </Section>
          </TabsContent>
        </div>
      </Tabs>
    </Page>
  );
}
