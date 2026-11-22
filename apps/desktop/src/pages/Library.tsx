import { useEffect, useMemo, useRef, useState } from "react";
import { getRouteApi, useNavigate } from "@tanstack/react-router";
import { useVirtualizer } from "@tanstack/react-virtual";
import { useTranslation } from "react-i18next";
import { ArrowDown, ArrowUp, Search, X } from "lucide-react";
import type { Difficulty, ProblemMode, ProblemSummary, StatusFilter } from "@/bindings";
import { Page } from "@/components/Page";
import { PatternLearnLinks } from "@/components/PatternLearnLinks";
import { ErrorNotice } from "@/components/ErrorNotice";
import { DifficultyLabel, FixLabel, ModeLabel, PackLabel, StatusIcon, usePatternLabel, useTagLabel } from "@/components/Semantic";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import type { LibrarySearch, SortKey } from "@/app/router";
import { useProblems } from "@/lib/queries";
import { ListsView } from "@/practice/ListsView";
import { AddToListMenu } from "@/practice/shared";
import { cn } from "@/lib/utils";

const route = getRouteApi("/library");
const ROW_HEIGHT = 40;
const ANY = "__any";
const DIFFICULTY_ORDER: Record<Difficulty, number> = { easy: 0, medium: 1, hard: 2 };
const COLUMNS = "grid-cols-[48px_minmax(0,2fr)_120px_minmax(0,1fr)_minmax(0,1fr)_36px]";

function sortProblems(list: ProblemSummary[], sort: SortKey, desc: boolean, label: (p: string) => string) {
  const key = (p: ProblemSummary): string | number => {
    switch (sort) {
      case "title":
        return p.title.toLocaleLowerCase();
      case "difficulty":
        return DIFFICULTY_ORDER[p.difficulty];
      case "pattern":
        return label(p.pattern).toLocaleLowerCase();
      default:
        return p.rank;
    }
  };
  const sorted = [...list].sort((a, b) => {
    const [x, y] = [key(a), key(b)];
    const c = x < y ? -1 : x > y ? 1 : a.id.localeCompare(b.id);
    return desc ? -c : c;
  });
  return sorted;
}

export function Library() {
  const { t, i18n } = useTranslation();
  const search = route.useSearch();
  const navigate = useNavigate({ from: "/library" });
  const patternLabel = usePatternLabel();
  const tagLabel = useTagLabel();

  const setSearch = (patch: Partial<LibrarySearch>) =>
    void navigate({ search: (prev) => ({ ...prev, ...patch }), replace: true });

  // Debounced free-text search (FTS in Rust).
  const [query, setQuery] = useState(search.q ?? "");
  useEffect(() => setQuery(search.q ?? ""), [search.q]);
  useEffect(() => {
    const id = window.setTimeout(() => {
      if ((search.q ?? "") !== query.trim()) setSearch({ q: query.trim() || undefined });
    }, 200);
    return () => window.clearTimeout(id);
  }, [query]);

  const all = useProblems();
  const filtered = useProblems({
    difficulty: search.difficulty ?? null,
    status: search.status ?? null,
    pattern: search.pattern ?? null,
    tag: search.tag ?? null,
    mode: search.mode ?? null,
    query: search.q ?? null,
  });

  const patterns = useMemo(
    () => [...new Set((all.data ?? []).map((p) => p.pattern))].sort(),
    [all.data],
  );
  const tags = useMemo(
    () => [...new Set((all.data ?? []).flatMap((p) => p.tags))].sort(),
    [all.data],
  );
  const sort = search.sort ?? "rank";
  const rows = useMemo(() => {
    // The kind filter (expansion M6) is applied here: `fix` is on every summary.
    const kind = search.kind;
    const source = search.source;
    const kept = (filtered.data ?? [])
      .filter((p) => !kind || p.fix === (kind === "fix"))
      .filter((p) => !source || (p.pack !== null) === (source === "packs"));
    return sortProblems(kept, sort, search.desc ?? false, patternLabel);
  }, [filtered.data, sort, search.desc, search.kind, search.source]);

  const searchRef = useRef<HTMLInputElement>(null);
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;
      const typing = target?.closest("input, textarea, select, [contenteditable=true]");
      if (e.key === "/" && !typing) {
        e.preventDefault();
        searchRef.current?.focus();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  const hasFilters = Boolean(
    search.q || search.difficulty || search.status || search.pattern || search.tag || search.mode || search.kind || search.source,
  );
  const clear = () => {
    setQuery("");
    void navigate({ search: { sort: search.sort, desc: search.desc }, replace: true });
  };

  const viewTabs = (
    <ToggleGroup
      type="single"
      variant="outline"
      size="sm"
      aria-label={t("library.title")}
      value={search.view ?? "problems"}
      onValueChange={(v) => v && setSearch({ view: v === "lists" ? "lists" : undefined, list: undefined })}
      className="mb-3"
    >
      <ToggleGroupItem value="problems" className="px-3">
        {t("practice.lists.tabProblems")}
      </ToggleGroupItem>
      <ToggleGroupItem value="lists" className="px-3">
        {t("practice.lists.tab")}
      </ToggleGroupItem>
    </ToggleGroup>
  );

  if (search.view === "lists") {
    return (
      <Page title={t("library.title")}>
        {viewTabs}
        <ListsView openId={search.list} onOpen={(id) => setSearch({ list: id })} />
      </Page>
    );
  }

  return (
    <Page
      title={t("library.title")}
      actions={
        <span className="text-sm text-muted-foreground" aria-live="polite">
          {filtered.data ? t("library.count", { count: rows.length }) : t("common.loading")}
        </span>
      }
    >
      {viewTabs}
      <div className="flex flex-wrap items-center gap-2" role="toolbar">
        <div className="relative w-64">
          <Search aria-hidden className="absolute top-2.5 left-2.5 size-4 text-muted-foreground" />
          <Input
            ref={searchRef}
            type="search"
            aria-label={t("library.search")}
            placeholder={`${t("library.search")} (/)`}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="pl-8"
          />
        </div>
        <ToggleGroup
          type="single"
          variant="outline"
          size="sm"
          aria-label={t("difficulty.label")}
          value={search.difficulty ?? ""}
          onValueChange={(v) => setSearch({ difficulty: (v || undefined) as Difficulty | undefined })}
        >
          {(["easy", "medium", "hard"] as const).map((d) => (
            <ToggleGroupItem key={d} value={d} aria-label={t(`difficulty.${d}`)}>
              <DifficultyLabel difficulty={d} />
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
        <ToggleGroup
          type="single"
          variant="outline"
          size="sm"
          aria-label={t("mode.label")}
          value={search.mode ?? ""}
          onValueChange={(v) => setSearch({ mode: (v || undefined) as ProblemMode | undefined })}
          data-testid="mode-filter"
        >
          {(["function", "program", "sql"] as const).map((m) => (
            <ToggleGroupItem key={m} value={m} aria-label={t(`mode.${m}`)}>
              {t(`mode.${m}`)}
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
        <ToggleGroup
          type="single"
          variant="outline"
          size="sm"
          aria-label={t("fix.kindLabel")}
          value={search.kind ?? ""}
          onValueChange={(v) => setSearch({ kind: (v || undefined) as "solve" | "fix" | undefined })}
          data-testid="kind-filter"
        >
          {(["solve", "fix"] as const).map((k) => (
            <ToggleGroupItem key={k} value={k} aria-label={t(`fix.kind.${k}`)}>
              {t(`fix.kind.${k}`)}
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
        {all.data?.some((p) => p.pack !== null) && (
          <ToggleGroup
            type="single"
            variant="outline"
            size="sm"
            aria-label={t("packs.sourceLabel")}
            value={search.source ?? ""}
            onValueChange={(v) => setSearch({ source: (v || undefined) as "builtin" | "packs" | undefined })}
            data-testid="source-filter"
          >
            {(["builtin", "packs"] as const).map((k) => (
              <ToggleGroupItem key={k} value={k} aria-label={t(`packs.source.${k}`)}>
                {t(`packs.source.${k}`)}
              </ToggleGroupItem>
            ))}
          </ToggleGroup>
        )}
        <FilterSelect
          label={t("status.label")}
          anyLabel={t("status.any")}
          value={search.status}
          options={(["todo", "attempted", "solved"] as const).map((s) => [s, t(`status.${s}`)])}
          onChange={(v) => setSearch({ status: v as StatusFilter | undefined })}
        />
        <FilterSelect
          label={t("pattern.label")}
          anyLabel={t("pattern.any")}
          value={search.pattern}
          options={patterns.map((p) => [p, patternLabel(p)])}
          onChange={(v) => setSearch({ pattern: v })}
        />
        <FilterSelect
          label={t("tag.label")}
          anyLabel={t("tag.any")}
          value={search.tag}
          options={tags
            .map((tag): [string, string] => [tag, tagLabel(tag)])
            .sort((a, b) => a[1].localeCompare(b[1], i18n.language))}
          onChange={(v) => setSearch({ tag: v })}
        />
        {hasFilters && (
          <Button variant="ghost" size="sm" onClick={clear}>
            <X aria-hidden />
            {t("library.clear")}
          </Button>
        )}
      </div>

      {search.pattern && <PatternLearnLinks pattern={search.pattern} className="mb-3" />}

      {filtered.error ? (
        <ErrorNotice error={filtered.error} onRetry={() => void filtered.refetch()} />
      ) : (
        <ProblemGrid
          rows={rows}
          loading={filtered.isPending}
          sort={sort}
          desc={search.desc ?? false}
          onSort={(key) =>
            setSearch(
              key === sort
                ? { desc: search.desc ? undefined : true }
                : { sort: key === "rank" ? undefined : key, desc: undefined },
            )
          }
          onClear={hasFilters ? clear : undefined}
        />
      )}
    </Page>
  );
}

function FilterSelect({
  label,
  anyLabel,
  value,
  options,
  onChange,
}: {
  label: string;
  anyLabel: string;
  value: string | undefined;
  options: [string, string][];
  onChange: (v: string | undefined) => void;
}) {
  return (
    <Select value={value ?? ANY} onValueChange={(v) => onChange(v === ANY ? undefined : v)}>
      <SelectTrigger size="sm" aria-label={label} className="min-w-36">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={ANY}>{anyLabel}</SelectItem>
        {options.map(([v, text]) => (
          <SelectItem key={v} value={v}>
            {text}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

function ProblemGrid({
  rows,
  loading,
  sort,
  desc,
  onSort,
  onClear,
}: {
  rows: ProblemSummary[];
  loading: boolean;
  sort: SortKey;
  desc: boolean;
  onSort: (key: SortKey) => void;
  onClear?: () => void;
}) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const patternLabel = usePatternLabel();
  const tagLabel = useTagLabel();
  const scrollRef = useRef<HTMLDivElement>(null);
  const [active, setActive] = useState(0);
  const virtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => scrollRef.current,
    estimateSize: () => ROW_HEIGHT,
    overscan: 10,
    initialRect: { width: 800, height: 600 },
  });

  useEffect(() => {
    if (active >= rows.length) setActive(Math.max(0, rows.length - 1));
  }, [rows.length, active]);

  const open = (p: ProblemSummary) =>
    void navigate({ to: "/problems/$problemId", params: { problemId: p.id } });

  const move = (index: number) => {
    const next = Math.min(Math.max(index, 0), rows.length - 1);
    setActive(next);
    virtualizer.scrollToIndex(next, { align: "auto" });
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (rows.length === 0) return;
    const page = Math.max(1, Math.floor((scrollRef.current?.clientHeight ?? 400) / ROW_HEIGHT) - 1);
    const keys: Record<string, () => void> = {
      ArrowDown: () => move(active + 1),
      ArrowUp: () => move(active - 1),
      PageDown: () => move(active + page),
      PageUp: () => move(active - page),
      Home: () => move(0),
      End: () => move(rows.length - 1),
      Enter: () => rows[active] && open(rows[active]),
    };
    const action = keys[e.key];
    if (action) {
      e.preventDefault();
      action();
    }
  };

  const header = (key: SortKey | null, label: string) =>
    key ? (
      <div
        role="columnheader"
        aria-sort={sort === key ? (desc ? "descending" : "ascending") : "none"}
      >
        <button
          type="button"
          className="inline-flex items-center gap-1 rounded-sm hover:text-foreground"
          onClick={() => onSort(key)}
          aria-label={t("library.sortBy", { column: label })}
        >
          {label}
          {sort === key && (desc ? <ArrowDown className="size-3" /> : <ArrowUp className="size-3" />)}
        </button>
      </div>
    ) : (
      <div role="columnheader">{label}</div>
    );

  const activeRow = rows[active];
  return (
    <div className="flex min-h-0 flex-1 flex-col rounded-lg border bg-card">
      <div
        role="grid"
        aria-label={t("library.table")}
        aria-rowcount={rows.length + 1}
        aria-activedescendant={activeRow ? `problem-row-${activeRow.id}` : undefined}
        tabIndex={0}
        onKeyDown={onKeyDown}
        className="flex min-h-0 flex-1 flex-col rounded-lg focus-visible:outline-offset-0"
      >
        <div role="rowgroup">
          <div
            role="row"
            aria-rowindex={1}
            className={cn(
              "grid items-center gap-3 border-b px-3 py-2 text-xs font-medium text-muted-foreground",
              COLUMNS,
            )}
          >
            {header(null, t("library.columns.status"))}
            {header("title", t("library.columns.title"))}
            {header("difficulty", t("library.columns.difficulty"))}
            {header("pattern", t("library.columns.pattern"))}
            {header(null, t("library.columns.tags"))}
            <span role="columnheader" className="sr-only">
              {t("practice.lists.addTo")}
            </span>
          </div>
        </div>

        {loading ? (
          <div className="space-y-2 p-3" aria-busy="true" aria-label={t("common.loading")}>
            {Array.from({ length: 6 }, (_, i) => (
              <Skeleton key={i} className="h-7 w-full" />
            ))}
          </div>
        ) : rows.length === 0 ? (
          <div className="flex flex-1 flex-col items-center justify-center gap-3 p-10 text-muted-foreground">
            <p>{t("library.empty")}</p>
            {onClear && (
              <Button variant="outline" size="sm" onClick={onClear}>
                {t("library.clear")}
              </Button>
            )}
          </div>
        ) : (
          <div ref={scrollRef} role="rowgroup" className="min-h-0 flex-1 overflow-auto">
            <div style={{ height: virtualizer.getTotalSize(), position: "relative" }}>
              {virtualizer.getVirtualItems().map((item) => {
                const p = rows[item.index];
                const selected = item.index === active;
                return (
                  <div
                    key={p.id}
                    id={`problem-row-${p.id}`}
                    role="row"
                    aria-rowindex={item.index + 2}
                    aria-selected={selected}
                    onClick={() => setActive(item.index)}
                    onDoubleClick={() => open(p)}
                    className={cn(
                      "absolute inset-x-0 grid cursor-default items-center gap-3 border-b px-3 text-sm",
                      COLUMNS,
                      selected ? "bg-accent" : "hover:bg-accent/60",
                    )}
                    style={{ height: ROW_HEIGHT, transform: `translateY(${item.start}px)` }}
                  >
                    <div role="gridcell">
                      <StatusIcon status={p.status} />
                    </div>
                    <div role="gridcell" className="flex min-w-0 items-center gap-2 font-medium">
                      <span className="truncate">{p.title}</span>
                      <ModeLabel mode={p.mode} compact />
                      <FixLabel fix={p.fix} />
                      <PackLabel pack={p.pack} />
                    </div>
                    <div role="gridcell">
                      <DifficultyLabel difficulty={p.difficulty} />
                    </div>
                    <div role="gridcell" className="truncate text-muted-foreground">
                      {patternLabel(p.pattern)}
                    </div>
                    <div role="gridcell" className="truncate text-xs text-muted-foreground">
                      {p.tags.map(tagLabel).join(", ")}
                    </div>
                    <div role="gridcell" onDoubleClick={(e) => e.stopPropagation()}>
                      <AddToListMenu problem={p.id} />
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        )}
      </div>
      <p className="border-t px-3 py-1.5 text-xs text-muted-foreground">{t("library.keyboardHint")}</p>
    </div>
  );
}
