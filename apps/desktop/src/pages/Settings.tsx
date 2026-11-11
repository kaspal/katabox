import { useTranslation } from "react-i18next";
import { CheckCircle2, Monitor, Moon, Sun, XCircle } from "lucide-react";
import { Page } from "@/components/Page";
import { ErrorNotice } from "@/components/ErrorNotice";
import { Badge } from "@/components/ui/badge";
import { MiddleEllipsis } from "@/components/MiddleEllipsis";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { UI_LANGS, type UiLangCode } from "@/i18n";
import { THEMES, usePrefs, type ThemePref } from "@/lib/prefs";
import { useAppInfo } from "@/lib/queries";
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

        <Section title={t("settings.about")}>
          {info.isPending ? (
            <Skeleton className="h-32 w-full" />
          ) : info.error ? (
            <ErrorNotice error={info.error} onRetry={() => void info.refetch()} />
          ) : (
            <>
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
              <h3 className="pt-2 text-sm font-semibold">{t("settings.toolchains")}</h3>
              <table className="w-full text-sm">
                <thead className="text-left text-xs text-muted-foreground">
                  <tr>
                    <th className="w-32 py-1 pr-4 font-medium">{t("settings.toolchainLanguage")}</th>
                    <th className="w-32 py-1 pr-4 font-medium">{t("settings.toolchainStatus")}</th>
                    <th className="py-1 font-medium">{t("settings.toolchainPath")}</th>
                  </tr>
                </thead>
                <tbody>
                  {info.data.toolchains.map((tc) => (
                    <tr key={tc.language} className="border-t">
                      <td className="py-1.5 pr-4">{t(`languages.${tc.language}`)}</td>
                      <td className="py-1.5 pr-4 whitespace-nowrap">
                        {tc.present ? (
                          <span className="inline-flex items-center gap-1 text-verdict-accepted">
                            <CheckCircle2 aria-hidden className="size-4" />
                            {t("settings.present")}
                          </span>
                        ) : (
                          <span className="inline-flex items-center gap-1 text-verdict-wrong">
                            <XCircle aria-hidden className="size-4" />
                            {t("settings.missing")}
                          </span>
                        )}
                      </td>
                      <td className="py-1.5 font-mono text-xs text-muted-foreground">
                        <MiddleEllipsis text={tc.program ?? tc.problem ?? ""} max={48} />
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </>
          )}
        </Section>
      </div>
    </Page>
  );
}
