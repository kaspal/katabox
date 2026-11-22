// "Search everything" (expansion M4, Ctrl+Shift+P): one box over problems and Learn
// topics (full-text, kb-app), your snippets, the settings and the user guide. All local.
import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { BookOpenText, Code2, GraduationCap, ScrollText, Settings2 } from "lucide-react";
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { DifficultyLabel } from "@/components/Semantic";
import { normalizeForSearch } from "@/app/paletteFilter";
import { api } from "@/lib/api";
import { activeEditor, insertIntoEditor } from "@/lib/editorBridge";
import type { DocSection } from "@/lib/docs";
import { usePrefs } from "@/lib/prefs";
import { SETTINGS_SEARCH_INDEX } from "@/lib/settingsIndex";

const EVENT = "katabox:search-everything";

export function openSearchEverything() {
  window.dispatchEvent(new Event(EVENT));
}

export function onOpenSearchEverything(handler: () => void): () => void {
  window.addEventListener(EVENT, handler);
  return () => window.removeEventListener(EVENT, handler);
}

function useDebounced<T>(value: T, ms: number): T {
  const [v, setV] = useState(value);
  useEffect(() => {
    const id = window.setTimeout(() => setV(value), ms);
    return () => window.clearTimeout(id);
  }, [value, ms]);
  return v;
}

export function SearchEverything({ open, onOpenChange }: { open: boolean; onOpenChange: (open: boolean) => void }) {
  const { t, i18n } = useTranslation();
  const { lang } = usePrefs();
  const navigate = useNavigate();
  const [search, setSearch] = useState("");
  const [docs, setDocs] = useState<DocSection[]>([]);
  useEffect(() => {
    if (open) setSearch("");
  }, [open]);
  // The guide loads only when this dialog is first used.
  useEffect(() => {
    if (!open) return;
    let live = true;
    void import("@/lib/docs").then((m) => live && setDocs(m.docSections(i18n.language)));
    return () => {
      live = false;
    };
  }, [open, i18n.language]);
  const q = useDebounced(search.trim(), 200);
  const results = useQuery({
    queryKey: ["search-all", q, lang],
    queryFn: () => api.searchAll(q, lang),
    enabled: open && q.length > 0,
  });
  const needle = normalizeForSearch(q);
  const settings = useMemo(
    () =>
      needle
        ? SETTINGS_SEARCH_INDEX.flatMap(([section, keys]) =>
            keys
              .map((k) => ({ section, label: t(k as "theme.label") }))
              .filter((x) => normalizeForSearch(x.label).includes(needle)),
          ).slice(0, 8)
        : [],
    [needle, t],
  );
  const helpHits = useMemo(
    () =>
      needle
        ? docs
            .filter((d) => normalizeForSearch(d.title).includes(needle) || normalizeForSearch(d.body).includes(needle))
            .slice(0, 6)
        : [],
    [needle, docs],
  );
  const run = (action: () => void) => {
    onOpenChange(false);
    action();
  };
  const r = results.data;
  const editor = activeEditor();
  return (
    <CommandDialog
      open={open}
      onOpenChange={onOpenChange}
      title={t("search.title")}
      description={t("search.placeholder")}
      filter={() => 1}
    >
      <CommandInput placeholder={t("search.placeholder")} value={search} onValueChange={setSearch} />
      <CommandList>
        {q.length > 0 && <CommandEmpty>{results.isFetching ? t("search.searching") : t("palette.empty")}</CommandEmpty>}
        {(r?.problems.length ?? 0) > 0 && (
          <CommandGroup heading={t("palette.problems")}>
            {r!.problems.map((p) => (
              <CommandItem
                key={p.id}
                value={`problem ${p.id}`}
                onSelect={() => run(() => void navigate({ to: "/problems/$problemId", params: { problemId: p.id } }))}
              >
                <Code2 aria-hidden />
                <span className="flex-1">{p.title}</span>
                <DifficultyLabel difficulty={p.difficulty} className="text-xs" />
              </CommandItem>
            ))}
          </CommandGroup>
        )}
        {(r?.topics.length ?? 0) > 0 && (
          <CommandGroup heading={t("palette.topics")}>
            {r!.topics.map((tp) => (
              <CommandItem
                key={tp.id}
                value={`topic ${tp.id}`}
                onSelect={() =>
                  run(() => void navigate({ to: "/learn/$subject/$topic", params: { subject: tp.subject, topic: tp.id } }))
                }
              >
                <GraduationCap aria-hidden />
                {t("palette.topic", { title: tp.title })}
              </CommandItem>
            ))}
          </CommandGroup>
        )}
        {(r?.snippets.length ?? 0) > 0 && (
          <CommandGroup heading={t("search.snippets")}>
            {r!.snippets.map((s) => {
              const insertable = editor !== null && editor.language === s.language;
              return (
                <CommandItem
                  key={s.uid}
                  value={`snippet ${s.uid}`}
                  onSelect={() =>
                    run(() => {
                      if (!(insertable && insertIntoEditor(s.body))) {
                        void navigate({ to: "/settings", search: { section: "editor" } });
                      }
                    })
                  }
                >
                  <ScrollText aria-hidden />
                  <span className="flex-1">{s.name}</span>
                  <span className="text-xs text-muted-foreground">
                    {t(`languages.${s.language}` as "languages.python")}
                    {insertable && ` · ${t("search.insert")}`}
                  </span>
                </CommandItem>
              );
            })}
          </CommandGroup>
        )}
        {settings.length > 0 && (
          <CommandGroup heading={t("nav.settings")}>
            {settings.map((s) => (
              <CommandItem
                key={`${s.section}-${s.label}`}
                value={`setting ${s.section} ${s.label}`}
                onSelect={() => run(() => void navigate({ to: "/settings", search: { section: s.section } }))}
              >
                <Settings2 aria-hidden />
                <span className="flex-1">{s.label}</span>
                <span className="text-xs text-muted-foreground">{t(`settings.sections.${s.section}`)}</span>
              </CommandItem>
            ))}
          </CommandGroup>
        )}
        {helpHits.length > 0 && (
          <CommandGroup heading={t("help.title")}>
            {helpHits.map((d) => (
              <CommandItem
                key={d.id}
                value={`help ${d.id}`}
                onSelect={() => run(() => void navigate({ to: "/help", search: { section: d.id } }))}
              >
                <BookOpenText aria-hidden />
                {d.title}
              </CommandItem>
            ))}
          </CommandGroup>
        )}
      </CommandList>
    </CommandDialog>
  );
}
