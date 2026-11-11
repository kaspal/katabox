import { useEffect, useRef, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { History, RotateCcw } from "lucide-react";
import { EditorState } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { MergeView } from "@codemirror/merge";
import type { Language, Snapshot } from "@/bindings";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { ErrorNotice } from "@/components/ErrorNotice";
import { VerdictBadge } from "@/components/Verdict";
import { api } from "@/lib/api";
import { cn } from "@/lib/utils";
import { relativeTime } from "@/lib/time";
import { baseExtensions, themeExtension } from "@/workspace/editor/setup";
import { loadLanguageExtension } from "@/workspace/editor/languages";

/** Snapshots with verdict badges; compare any two (or one with the current code); restore. */
export function HistoryTab({
  problemId,
  currentCode,
  currentLanguage,
  onRestored,
}: {
  problemId: string;
  currentCode: string | null;
  currentLanguage: Language | null;
  onRestored: (language: Language) => void;
}) {
  const { t, i18n } = useTranslation();
  const queryClient = useQueryClient();
  const [selected, setSelected] = useState<number[]>([]);
  const list = useQuery({
    queryKey: ["snapshots", problemId],
    queryFn: () => api.listSnapshots(problemId),
  });
  const restore = useMutation({
    mutationFn: (id: number) => api.restoreSnapshot(id),
    onSuccess: (snap) => {
      void queryClient.invalidateQueries({ queryKey: ["snapshots", problemId] });
      setSelected([]);
      onRestored(snap.language);
    },
  });

  const toggle = (id: number) =>
    setSelected((s) => (s.includes(id) ? s.filter((x) => x !== id) : [...s, id].slice(-2)));

  if (list.isPending) return <Skeleton className="h-32 w-full" />;
  if (list.error) return <ErrorNotice error={list.error} onRetry={() => void list.refetch()} />;
  if (list.data.length === 0) {
    return <p className="text-sm text-muted-foreground">{t("workspace.history.none")}</p>;
  }

  const byId = new Map(list.data.map((s) => [s.id, s]));
  const picked = selected.map((id) => byId.get(id)).filter((s): s is Snapshot => !!s);
  // Older on the left. With one selected, compare it with the code in the editor.
  const [left, right] =
    picked.length === 2
      ? [...picked].sort((a, b) => a.id - b.id)
      : picked.length === 1 && currentCode !== null
        ? [picked[0], null]
        : [null, null];

  return (
    <div className="space-y-3">
      <p className="text-xs text-muted-foreground">{t("workspace.history.help")}</p>
      <ul className="divide-y rounded-lg border" data-testid="history-list">
        {list.data.map((s) => (
          <li key={s.id} className={cn("flex items-center gap-2 px-3 py-2", selected.includes(s.id) && "bg-accent")}>
            <input
              type="checkbox"
              className="size-4 accent-primary"
              checked={selected.includes(s.id)}
              onChange={() => toggle(s.id)}
              aria-label={t("workspace.history.select", { when: relativeTime(s.created_at, i18n.language) })}
            />
            <div className="min-w-0 flex-1">
              <div className="flex flex-wrap items-center gap-2 text-sm">
                {s.verdict ? <VerdictBadge verdict={s.verdict} /> : <History aria-hidden className="size-4 text-muted-foreground" />}
                <span className="text-xs text-muted-foreground">
                  {t(`workspace.history.source.${s.source}`)} · {t(`languages.${s.language}`)} ·{" "}
                  {relativeTime(s.created_at, i18n.language)}
                </span>
              </div>
            </div>
            <Button
              size="sm"
              variant="ghost"
              disabled={restore.isPending}
              onClick={() => restore.mutate(s.id)}
              aria-label={t("workspace.history.restoreLabel", { when: relativeTime(s.created_at, i18n.language) })}
            >
              <RotateCcw aria-hidden />
              {t("workspace.history.restore")}
            </Button>
          </li>
        ))}
      </ul>
      {restore.error && <ErrorNotice error={restore.error} />}
      {left && (
        <div className="space-y-1">
          <div className="grid grid-cols-2 text-xs text-muted-foreground">
            <span>{relativeTime(left.created_at, i18n.language)}</span>
            <span>{right ? relativeTime(right.created_at, i18n.language) : t("workspace.history.current")}</span>
          </div>
          <DiffView
            a={left.code}
            b={right ? right.code : (currentCode ?? "")}
            language={right?.language ?? (currentLanguage ?? left.language)}
          />
        </div>
      )}
    </div>
  );
}

function DiffView({ a, b, language }: { a: string; b: string; language: Language }) {
  const host = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!host.current) return;
    let view: MergeView | null = null;
    let live = true;
    const parent = host.current;
    void loadLanguageExtension(language).then((lang) => {
      if (!live) return;
      const ext = [
        baseExtensions,
        lang,
        themeExtension(13),
        EditorView.editable.of(false),
        EditorState.readOnly.of(true),
      ];
      view = new MergeView({
        a: { doc: a, extensions: ext },
        b: { doc: b, extensions: ext },
        parent,
        collapseUnchanged: { margin: 3, minSize: 6 },
      });
    });
    return () => {
      live = false;
      view?.destroy();
    };
  }, [a, b, language]);
  return <div ref={host} className="max-h-96 overflow-auto rounded-md border" data-testid="history-diff" />;
}
