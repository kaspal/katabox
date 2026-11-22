import { useEffect, useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { BarChart3, GraduationCap, Keyboard, Languages, Monitor, Moon, ScrollText, Search, Settings2, Square, Sun, Timer } from "lucide-react";
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { DifficultyLabel, useTagLabel } from "@/components/Semantic";
import { DEV_NAV, NAV, WORKSPACE_ICON } from "@/app/nav";
import { openShortcuts } from "@/app/shortcuts";
import { openSearchEverything } from "@/app/SearchEverything";
import { activeEditor, insertIntoEditor, onActiveEditor } from "@/lib/editorBridge";
import { useSnippets } from "@/practice/snippets";
import { useFocusActions, useFocusBreak, useFocusCurrent, useFocusWork } from "@/practice/focus";
import { paletteFilter } from "@/app/paletteFilter";
import { PROGRESS_TABS, SETTINGS_SECTIONS } from "@/lib/pageTabs";
import { UI_LANGS } from "@/i18n";
import { THEMES, usePrefs, type ThemePref } from "@/lib/prefs";
import { useLearnCatalog, useProblems } from "@/lib/queries";

const THEME_ICON: Record<ThemePref, typeof Sun> = { system: Monitor, light: Sun, dark: Moon };

/** Ctrl+K: go to a page (or a Progress tab, a Settings section), open a problem, switch
 * theme or UI language, see the shortcuts. Every command is found by its label as shown,
 * with or without punctuation and accents (`paletteFilter`). */
export function CommandPalette({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { theme, uiLang, setTheme, setUiLang } = usePrefs();
  const problems = useProblems();
  const learn = useLearnCatalog();
  const tagLabel = useTagLabel();
  const snippets = useSnippets();
  const focusNow = useFocusCurrent();
  const focus = useFocusActions();
  const [focusWork] = useFocusWork();
  const [focusBreak] = useFocusBreak();
  // Snippets for the open editor's language, inserted at its cursor.
  const [editor, setEditor] = useState(activeEditor());
  useEffect(() => onActiveEditor(() => setEditor(activeEditor())), []);
  const insertable = editor ? (snippets.data ?? []).filter((s) => s.language === editor.language) : [];
  // Each opening starts empty, even when reopened during the closing animation.
  const [search, setSearch] = useState("");
  useEffect(() => {
    if (open) setSearch("");
  }, [open]);

  const run = (action: () => void) => {
    onOpenChange(false);
    action();
  };

  return (
    <CommandDialog
      open={open}
      onOpenChange={onOpenChange}
      title={t("palette.open")}
      description={t("palette.placeholder")}
      filter={paletteFilter}
    >
      <CommandInput placeholder={t("palette.placeholder")} value={search} onValueChange={setSearch} />
      <CommandList>
        <CommandEmpty>{t("palette.empty")}</CommandEmpty>
        <CommandGroup heading={t("palette.pages")}>
          {[...NAV, ...DEV_NAV].map((item) => (
            <CommandItem
              key={item.to}
              value={`page ${t(item.labelKey)}`}
              onSelect={() => run(() => void navigate({ to: item.to }))}
            >
              <item.icon aria-hidden />
              {t(item.labelKey)}
            </CommandItem>
          ))}
          {PROGRESS_TABS.map((tab) => {
            const label = `${t("nav.progress")}: ${t(`progress.tabs.${tab}`)}`;
            return (
              <CommandItem
                key={`progress-${tab}`}
                value={`page ${label}`}
                onSelect={() => run(() => void navigate({ to: "/progress", search: { tab } }))}
              >
                <BarChart3 aria-hidden />
                {label}
              </CommandItem>
            );
          })}
          {SETTINGS_SECTIONS.map((section) => {
            const label = `${t("nav.settings")}: ${t(`settings.sections.${section}`)}`;
            return (
              <CommandItem
                key={`settings-${section}`}
                value={`page ${label}`}
                onSelect={() => run(() => void navigate({ to: "/settings", search: { section } }))}
              >
                <Settings2 aria-hidden />
                {label}
              </CommandItem>
            );
          })}
        </CommandGroup>
        <CommandGroup heading={t("palette.problems")}>
          {(problems.data ?? []).map((p) => (
            <CommandItem
              key={p.id}
              // Tags match by their names in the UI language too.
              value={`problem ${p.title} ${p.id} ${p.tags.map(tagLabel).join(" ")}`}
              onSelect={() =>
                run(() => void navigate({ to: "/problems/$problemId", params: { problemId: p.id } }))
              }
            >
              <WORKSPACE_ICON aria-hidden />
              <span className="flex-1">{p.title}</span>
              <DifficultyLabel difficulty={p.difficulty} className="text-xs" />
            </CommandItem>
          ))}
        </CommandGroup>
        <CommandGroup heading={t("palette.topics")}>
          {(learn.data ?? []).flatMap((s) =>
            s.topics.map((tp) => (
              <CommandItem
                key={tp.id}
                value={`learn ${t("nav.learn")} ${tp.title} ${tp.id} ${s.name}`}
                onSelect={() =>
                  run(() => void navigate({ to: "/learn/$subject/$topic", params: { subject: s.id, topic: tp.id } }))
                }
              >
                <GraduationCap aria-hidden />
                {t("palette.topic", { title: tp.title })}
              </CommandItem>
            )),
          )}
        </CommandGroup>
        <CommandGroup heading={t("focus.title")}>
          {focusNow.data ? (
            <CommandItem value={t("focus.paletteStop")} onSelect={() => run(() => focus.stop.mutate())}>
              <Square aria-hidden />
              {t("focus.paletteStop")}
            </CommandItem>
          ) : (
            <CommandItem
              value={t("focus.paletteStart", { work: focusWork, brk: focusBreak })}
              onSelect={() => run(() => focus.start.mutate({ work: focusWork, brk: focusBreak }))}
            >
              <Timer aria-hidden />
              {t("focus.paletteStart", { work: focusWork, brk: focusBreak })}
            </CommandItem>
          )}
        </CommandGroup>
        <CommandGroup heading={t("palette.theme")}>
          {THEMES.map((th) => {
            const Icon = THEME_ICON[th];
            return (
              <CommandItem
                key={th}
                value={`theme ${t("palette.theme")}: ${t(`theme.${th}`)}`}
                onSelect={() => run(() => setTheme(th))}
                data-checked={theme === th}
              >
                <Icon aria-hidden />
                {t("palette.theme")}: {t(`theme.${th}`)}
              </CommandItem>
            );
          })}
        </CommandGroup>
        <CommandGroup heading={t("palette.language")}>
          {UI_LANGS.map((l) => (
            <CommandItem
              key={l}
              value={`language ${t("palette.language")}: ${t(`language.${l}`)}`}
              onSelect={() => run(() => setUiLang(l))}
              data-checked={uiLang === l}
            >
              <Languages aria-hidden />
              {t("palette.language")}: {t(`language.${l}`)}
            </CommandItem>
          ))}
        </CommandGroup>
        {insertable.length > 0 && (
          <CommandGroup heading={t("search.snippets")}>
            {insertable.map((s) => (
              <CommandItem
                key={s.uid}
                value={`snippet ${t("palette.insertSnippet", { name: s.name })} ${s.tags.join(" ")}`}
                onSelect={() => run(() => void insertIntoEditor(s.body))}
              >
                <ScrollText aria-hidden />
                {t("palette.insertSnippet", { name: s.name })}
              </CommandItem>
            ))}
          </CommandGroup>
        )}
        <CommandGroup heading={t("palette.help")}>
          <CommandItem value={`help ${t("search.title")} Ctrl Shift P`} onSelect={() => run(openSearchEverything)}>
            <Search aria-hidden />
            {t("search.title")}
          </CommandItem>
          <CommandItem value={`help ${t("help.title")}`} onSelect={() => run(() => void navigate({ to: "/help" }))}>
            <GraduationCap aria-hidden />
            {t("help.title")}
          </CommandItem>
          <CommandItem value={`help ${t("shortcuts.open")} F1`} onSelect={() => run(openShortcuts)}>
            <Keyboard aria-hidden />
            {t("shortcuts.open")}
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  );
}
