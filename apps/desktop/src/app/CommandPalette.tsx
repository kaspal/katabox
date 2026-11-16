import { useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { GraduationCap, Keyboard, Languages, Monitor, Moon, Sun } from "lucide-react";
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
import { UI_LANGS } from "@/i18n";
import { THEMES, usePrefs, type ThemePref } from "@/lib/prefs";
import { useLearnCatalog, useProblems } from "@/lib/queries";

const THEME_ICON: Record<ThemePref, typeof Sun> = { system: Monitor, light: Sun, dark: Moon };

/** Ctrl+K: go to a page, open a problem, switch theme or UI language, see the shortcuts. */
export function CommandPalette({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { theme, lang, setTheme, setLang } = usePrefs();
  const problems = useProblems();
  const learn = useLearnCatalog();
  const tagLabel = useTagLabel();

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
    >
      <CommandInput placeholder={t("palette.placeholder")} />
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
        <CommandGroup heading={t("palette.theme")}>
          {THEMES.map((th) => {
            const Icon = THEME_ICON[th];
            return (
              <CommandItem
                key={th}
                value={`theme ${t("palette.theme")} ${t(`theme.${th}`)}`}
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
              value={`language ${t("palette.language")} ${t(`language.${l}`)}`}
              onSelect={() => run(() => setLang(l))}
              data-checked={lang === l}
            >
              <Languages aria-hidden />
              {t("palette.language")}: {t(`language.${l}`)}
            </CommandItem>
          ))}
        </CommandGroup>
        <CommandGroup heading={t("palette.help")}>
          <CommandItem value={`help ${t("shortcuts.open")} F1`} onSelect={() => run(openShortcuts)}>
            <Keyboard aria-hidden />
            {t("shortcuts.open")}
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  );
}
