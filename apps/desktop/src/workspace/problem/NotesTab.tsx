import { useEffect, useRef, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { Skeleton } from "@/components/ui/skeleton";
import { ErrorNotice } from "@/components/ErrorNotice";
import { api } from "@/lib/api";
import { CodeEditor } from "@/workspace/editor/CodeEditor";
import { Markdown } from "@/workspace/problem/Markdown";
import { SavedReflection } from "@/practice/journal";

export const NOTE_SAVE_MS = 500;

/** One Markdown note per problem, autosaved (enhancement 1). */
export function NotesTab({ problemId }: { problemId: string }) {
  const { t } = useTranslation();
  const q = useQuery({ queryKey: ["note", problemId], queryFn: () => api.getNote(problemId) });
  const [mode, setMode] = useState<"edit" | "preview">("edit");
  const [text, setText] = useState<string | null>(null);
  const [status, setStatus] = useState<"idle" | "saving" | "saved">("idle");
  const timer = useRef<number | undefined>(undefined);
  const pending = useRef<string | null>(null);

  useEffect(() => {
    if (q.data !== undefined && text === null) setText(q.data ?? "");
  }, [q.data, text]);

  const flush = async () => {
    window.clearTimeout(timer.current);
    const body = pending.current;
    pending.current = null;
    if (body === null) return;
    await api.setNote(problemId, body).catch(() => {});
    setStatus("saved");
  };

  // Save on leaving the problem.
  useEffect(() => () => void flush(),
  [problemId]);

  const onChange = (body: string) => {
    setText(body);
    pending.current = body;
    setStatus("saving");
    window.clearTimeout(timer.current);
    timer.current = window.setTimeout(() => void flush(), NOTE_SAVE_MS);
  };

  if (q.isPending || text === null) return <Skeleton className="h-32 w-full" />;
  if (q.error) return <ErrorNotice error={q.error} />;
  return (
    <div className="flex h-full min-h-64 flex-col gap-2">
      <SavedReflection problemId={problemId} />
      <div className="flex items-center justify-between">
        <ToggleGroup
          type="single"
          size="sm"
          variant="outline"
          value={mode}
          onValueChange={(v) => v && setMode(v as "edit" | "preview")}
          aria-label={t("workspace.notes.mode")}
        >
          <ToggleGroupItem value="edit" className="px-3">
            {t("workspace.notes.edit")}
          </ToggleGroupItem>
          <ToggleGroupItem value="preview" className="px-3">
            {t("workspace.notes.preview")}
          </ToggleGroupItem>
        </ToggleGroup>
        <span className="text-xs text-muted-foreground" aria-live="polite">
          {status === "saving" ? t("workspace.save.saving") : status === "saved" ? t("workspace.save.saved") : ""}
        </span>
      </div>
      {mode === "edit" ? (
        <div className="min-h-48 flex-1 overflow-hidden rounded-md border">
          <CodeEditor
            docKey={`note:${problemId}`}
            value={text}
            lang="markdown"
            lineNumbers={false}
            ariaLabel={t("workspace.notes.label")}
            onChange={onChange}
          />
        </div>
      ) : text.trim() ? (
        <Markdown text={text} className="rounded-md border p-3" />
      ) : (
        <p className="text-sm text-muted-foreground">{t("workspace.notes.empty")}</p>
      )}
    </div>
  );
}
