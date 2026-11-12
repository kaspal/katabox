// Personal lists (ADR 0019), the Library's Lists tab: list cards with progress; an open
// list shows its problems, reorderable by dragging or with Alt+↑ / Alt+↓.
import { useEffect, useState } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowLeft, GripVertical, MoreHorizontal, Plus } from "lucide-react";
import type { ListEntry, ListView } from "@/bindings";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Progress as Bar } from "@/components/ui/progress";
import { Skeleton } from "@/components/ui/skeleton";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, StatusIcon, usePatternLabel } from "@/components/Semantic";
import { useListActions, useLists } from "@/lib/practice";
import { cn } from "@/lib/utils";
import { ConfirmDialog, NameDialog } from "@/practice/shared";

export function ListsView({ openId, onOpen }: { openId?: number; onOpen: (id: number | undefined) => void }) {
  const { t } = useTranslation();
  const lists = useLists();
  const actions = useListActions();
  const [creating, setCreating] = useState(false);
  const [renaming, setRenaming] = useState<ListView | null>(null);
  const [deleting, setDeleting] = useState<ListView | null>(null);

  if (lists.isPending) return <Skeleton className="h-40 w-full max-w-3xl" />;
  if (lists.error) return <ErrorNotice error={lists.error} onRetry={() => void lists.refetch()} />;
  const all = lists.data ?? [];
  const open = all.find((l) => l.id === openId);

  const dialogs = (
    <>
      <NameDialog
        open={creating}
        title={t("practice.lists.new")}
        submitLabel={t("practice.lists.create")}
        onOpenChange={setCreating}
        onSubmit={(name) => actions.create.mutateAsync(name)}
      />
      <NameDialog
        open={renaming !== null}
        title={t("practice.lists.rename")}
        initial={renaming?.name ?? ""}
        submitLabel={t("practice.lists.save")}
        onOpenChange={(o) => !o && setRenaming(null)}
        onSubmit={(name) => actions.rename.mutateAsync({ id: renaming!.id, name })}
      />
      <ConfirmDialog
        open={deleting !== null}
        title={t("practice.lists.deleteTitle")}
        description={t("practice.lists.deleteConfirm", { name: deleting?.name ?? "" })}
        confirmLabel={t("practice.lists.delete")}
        onOpenChange={(o) => !o && setDeleting(null)}
        onConfirm={() => {
          if (deleting) {
            actions.remove.mutate(deleting.id);
            if (deleting.id === openId) onOpen(undefined);
          }
        }}
      />
    </>
  );

  const menu = (l: ListView) => (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon" className="size-8" aria-label={`${t("practice.lists.actions")}: ${l.name}`}>
          <MoreHorizontal aria-hidden />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onSelect={() => setRenaming(l)}>{t("practice.lists.rename")}</DropdownMenuItem>
        <DropdownMenuItem variant="destructive" onSelect={() => setDeleting(l)}>
          {t("practice.lists.delete")}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );

  if (open) {
    return (
      <div className="max-w-3xl space-y-3" data-testid="list-detail">
        {dialogs}
        <div className="flex items-center gap-2">
          <Button variant="ghost" size="sm" onClick={() => onOpen(undefined)}>
            <ArrowLeft aria-hidden />
            {t("practice.lists.back")}
          </Button>
          <h2 className="min-w-0 flex-1 truncate text-lg font-semibold">{open.name}</h2>
          <span className="text-sm tabular-nums text-muted-foreground">
            {t("practice.lists.progress", { solved: open.solved, total: open.total })}
          </span>
          {menu(open)}
        </div>
        {open.entries.length === 0 ? (
          <p className="text-sm text-muted-foreground">{t("practice.lists.emptyList")}</p>
        ) : (
          <>
            <OrderedEntries list={open} />
            <p className="text-xs text-muted-foreground">{t("practice.lists.reorderHint")}</p>
          </>
        )}
      </div>
    );
  }

  return (
    <div className="max-w-3xl space-y-3" data-testid="lists">
      {dialogs}
      <Button size="sm" onClick={() => setCreating(true)}>
        <Plus aria-hidden />
        {t("practice.lists.new")}
      </Button>
      {all.length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("practice.lists.empty")}</p>
      ) : (
        <ul className="grid gap-3 sm:grid-cols-2">
          {all.map((l) => (
            <li key={l.id} className="flex flex-col gap-2 rounded-lg border bg-card p-4" data-testid={`list-${l.id}`}>
              <div className="flex items-center gap-2">
                <button
                  type="button"
                  className="min-w-0 flex-1 truncate text-left font-semibold hover:underline"
                  onClick={() => onOpen(l.id)}
                >
                  {l.name}
                </button>
                {menu(l)}
              </div>
              <Bar
                value={l.total === 0 ? 0 : (100 * l.solved) / l.total}
                aria-label={`${l.name}: ${t("practice.lists.progress", { solved: l.solved, total: l.total })}`}
              />
              <span className="text-sm tabular-nums text-muted-foreground">
                {t("practice.lists.progress", { solved: l.solved, total: l.total })}
              </span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

/** A list's problems in order: drag a row, or focus it and press Alt+↑ / Alt+↓. */
function OrderedEntries({ list }: { list: ListView }) {
  const { t } = useTranslation();
  const patternLabel = usePatternLabel();
  const actions = useListActions();
  const [order, setOrder] = useState<ListEntry[]>(list.entries);
  const [dragging, setDragging] = useState<number | null>(null);
  const [focusId, setFocusId] = useState<string | null>(null);
  useEffect(() => setOrder(list.entries), [list.entries]);
  useEffect(() => {
    if (focusId) document.getElementById(`list-entry-${focusId}`)?.focus();
  }, [order, focusId]);

  const commit = (next: ListEntry[]) => {
    setOrder(next);
    actions.reorder.mutate({ id: list.id, problems: next.map((e) => e.problem_id) });
  };
  const move = (from: number, to: number) => {
    if (to < 0 || to >= order.length || from === to) return;
    const next = [...order];
    const [item] = next.splice(from, 1);
    next.splice(to, 0, item);
    commit(next);
  };

  return (
    <ol className="divide-y rounded-lg border" aria-label={list.name}>
      {order.map((e, i) => {
        const p = e.problem;
        return (
          <li
            key={e.problem_id}
            id={`list-entry-${e.problem_id}`}
            tabIndex={0}
            draggable
            data-testid={`entry-${e.problem_id}`}
            aria-label={p ? p.title : t("practice.lists.unavailable")}
            onDragStart={() => setDragging(i)}
            onDragOver={(ev) => ev.preventDefault()}
            onDrop={(ev) => {
              ev.preventDefault();
              if (dragging !== null) move(dragging, i);
              setDragging(null);
            }}
            onDragEnd={() => setDragging(null)}
            onKeyDown={(ev) => {
              if (ev.altKey && (ev.key === "ArrowUp" || ev.key === "ArrowDown")) {
                ev.preventDefault();
                setFocusId(e.problem_id);
                move(i, ev.key === "ArrowUp" ? i - 1 : i + 1);
              }
            }}
            className={cn(
              "flex items-center gap-3 px-3 py-2 text-sm outline-none focus-visible:bg-accent",
              dragging === i && "opacity-50",
            )}
          >
            <GripVertical aria-hidden className="size-4 shrink-0 cursor-grab text-muted-foreground" />
            {p ? (
              <>
                <StatusIcon status={p.status} />
                <Link
                  to="/problems/$problemId"
                  params={{ problemId: p.id }}
                  className="min-w-0 flex-1 truncate font-medium hover:underline"
                >
                  {p.title}
                </Link>
                <DifficultyLabel difficulty={p.difficulty} className="text-xs" />
                <span className="hidden w-36 truncate text-xs text-muted-foreground sm:inline">
                  {patternLabel(p.pattern)}
                </span>
              </>
            ) : (
              <span className="min-w-0 flex-1 truncate text-muted-foreground italic" data-testid="unavailable">
                {t("practice.lists.unavailable")} <span className="font-mono not-italic">{e.problem_id}</span>
              </span>
            )}
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon"
                  className="size-7"
                  aria-label={`${t("practice.lists.problemActions")}: ${p ? p.title : e.problem_id}`}
                >
                  <MoreHorizontal aria-hidden />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                {p && (
                  <>
                    <DropdownMenuItem disabled={i === 0} onSelect={() => move(i, i - 1)}>
                      {t("practice.lists.moveUp")}
                    </DropdownMenuItem>
                    <DropdownMenuItem disabled={i === order.length - 1} onSelect={() => move(i, i + 1)}>
                      {t("practice.lists.moveDown")}
                    </DropdownMenuItem>
                  </>
                )}
                <DropdownMenuItem
                  variant="destructive"
                  onSelect={() => actions.drop.mutate({ id: list.id, problem: e.problem_id })}
                >
                  {t("practice.lists.remove")}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </li>
        );
      })}
    </ol>
  );
}
