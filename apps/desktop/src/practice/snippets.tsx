// Settings → Editor → Snippets (expansion M4): your own code snippets per language,
// inserted with `/snip` in the editor or from the command palette.
import { useState } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { Pencil, Plus, ScrollText, Trash2 } from "lucide-react";
import type { Language, SnippetRow } from "@/bindings";
import { ErrorNotice } from "@/components/ErrorNotice";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { api } from "@/lib/api";
import { JUDGE_LANGUAGES } from "@/lib/settings";

export const snippetKeys = { all: ["snippets"] as const };

export function useSnippets() {
  return useQuery({ queryKey: snippetKeys.all, queryFn: () => api.snippets() });
}

type Draft = { uid: string | null; language: Language; name: string; tags: string; body: string };
const EMPTY: Draft = { uid: null, language: "python", name: "", tags: "", body: "" };

export function SnippetsPanel() {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const list = useSnippets();
  const [draft, setDraft] = useState<Draft | null>(null);
  const [error, setError] = useState<unknown>(null);
  const refresh = () => void queryClient.invalidateQueries({ queryKey: snippetKeys.all });
  const save = async () => {
    if (!draft) return;
    setError(null);
    try {
      await api.saveSnippet({
        uid: draft.uid,
        language: draft.language,
        name: draft.name,
        tags: draft.tags.split(",").map((x) => x.trim()).filter(Boolean),
        body: draft.body,
      });
      setDraft(null);
      refresh();
    } catch (e) {
      setError(e);
    }
  };
  const edit = (s: SnippetRow) =>
    setDraft({ uid: s.uid, language: s.language as Language, name: s.name, tags: s.tags.join(", "), body: s.body });
  return (
    <div className="space-y-3" data-testid="snippets-panel">
      <p className="text-sm text-muted-foreground">{t("snippets.intro")}</p>
      {list.error ? (
        <ErrorNotice error={list.error} />
      ) : (list.data ?? []).length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("snippets.none")}</p>
      ) : (
        <ul className="divide-y rounded-md border">
          {list.data!.map((s) => (
            <li key={s.uid} className="flex items-center gap-2 px-3 py-2 text-sm" data-testid="snippet-row">
              <ScrollText aria-hidden className="size-4 text-muted-foreground" />
              <span className="min-w-0 flex-1 truncate font-medium">{s.name}</span>
              <span className="text-xs text-muted-foreground">{t(`languages.${s.language}` as "languages.python")}</span>
              {s.tags.length > 0 && <span className="text-xs text-muted-foreground">{s.tags.join(", ")}</span>}
              <Button size="icon" variant="ghost" className="size-7" aria-label={t("snippets.edit", { name: s.name })} onClick={() => edit(s)}>
                <Pencil aria-hidden />
              </Button>
              <Button
                size="icon"
                variant="ghost"
                className="size-7"
                aria-label={t("snippets.delete", { name: s.name })}
                onClick={async () => {
                  await api.deleteSnippet(s.uid);
                  refresh();
                }}
              >
                <Trash2 aria-hidden />
              </Button>
            </li>
          ))}
        </ul>
      )}
      {draft ? (
        <div className="space-y-2 rounded-md border p-3" data-testid="snippet-form">
          <div className="grid grid-cols-[120px_1fr] items-center gap-2 text-sm">
            <label htmlFor="snip-name" className="text-muted-foreground">
              {t("snippets.name")}
            </label>
            <Input id="snip-name" value={draft.name} maxLength={80} onChange={(e) => setDraft({ ...draft, name: e.target.value })} />
            <span className="text-muted-foreground">{t("snippets.language")}</span>
            <Select value={draft.language} onValueChange={(v) => setDraft({ ...draft, language: v as Language })}>
              <SelectTrigger size="sm" className="w-44" aria-label={t("snippets.language")}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {JUDGE_LANGUAGES.map((l) => (
                  <SelectItem key={l} value={l}>
                    {t(`languages.${l}`)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <label htmlFor="snip-tags" className="text-muted-foreground">
              {t("snippets.tags")}
            </label>
            <Input id="snip-tags" value={draft.tags} placeholder={t("snippets.tagsHint")} onChange={(e) => setDraft({ ...draft, tags: e.target.value })} />
          </div>
          <label className="flex flex-col gap-1 text-sm text-muted-foreground">
            {t("snippets.body")}
            <textarea
              value={draft.body}
              rows={8}
              spellCheck={false}
              onChange={(e) => setDraft({ ...draft, body: e.target.value })}
              className="rounded-md border bg-background px-2 py-1 font-mono text-sm text-foreground"
            />
          </label>
          {error !== null && <ErrorNotice error={error} />}
          <div className="flex gap-2">
            <Button size="sm" onClick={() => void save()} disabled={!draft.name.trim()}>
              {t("snippets.save")}
            </Button>
            <Button size="sm" variant="ghost" onClick={() => setDraft(null)}>
              {t("snippets.cancel")}
            </Button>
          </div>
        </div>
      ) : (
        <Button size="sm" variant="outline" onClick={() => setDraft({ ...EMPTY })}>
          <Plus aria-hidden />
          {t("snippets.add")}
        </Button>
      )}
    </div>
  );
}
