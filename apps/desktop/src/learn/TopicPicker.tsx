// The topic picker (ADR 0029, mockup "Main", left): a filter, then every subject; the
// current subject lists its topics with a "Viewed" check. Subjects with no topics yet
// say "Coming soon" and are not links.
import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Check, Search } from "lucide-react";
import type { LearnSubject } from "@/bindings";
import { cn } from "@/lib/utils";

export function TopicPicker({
  subjects,
  current,
  viewed,
}: {
  subjects: LearnSubject[];
  current: { subject: string; topic: string };
  viewed: string[];
}) {
  const { t } = useTranslation();
  const [filter, setFilter] = useState("");
  const q = filter.trim().toLowerCase();
  const matches = (title: string) => q === "" || title.toLowerCase().includes(q);

  return (
    <nav aria-label={t("learn.picker.label")} className="flex h-full flex-col gap-3 overflow-auto p-3">
      <label className="flex h-9 items-center gap-2 rounded-lg border bg-card px-2.5 text-muted-foreground">
        <Search aria-hidden className="size-4 flex-none" />
        <input
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          placeholder={t("learn.picker.filter")}
          aria-label={t("learn.picker.filter")}
          className="w-full bg-transparent text-sm text-foreground outline-none"
        />
      </label>
      <ul className="flex flex-col gap-0.5 text-sm">
        {subjects.map((s) => {
          const topics = s.topics.filter((tp) => matches(tp.title));
          const open = s.id === current.subject || (q !== "" && topics.length > 0);
          if (q !== "" && topics.length === 0) return null;
          const first = s.topics[0];
          return (
            <li key={s.id}>
              {first ? (
                <Link
                  to="/learn/$subject/$topic"
                  params={{ subject: s.id, topic: first.id }}
                  aria-expanded={open}
                  className={cn(
                    "flex h-8 items-center justify-between rounded-md px-2 hover:bg-accent",
                    open && "font-semibold",
                  )}
                >
                  <span>{s.name}</span>
                  <span className="text-xs font-normal text-muted-foreground">
                    {s.topics.filter((tp) => viewed.includes(tp.id)).length} / {s.topics.length}
                  </span>
                </Link>
              ) : (
                <div className="flex min-h-8 items-center justify-between gap-2 px-2 py-1 text-muted-foreground">
                  <span className="min-w-0">{s.name}</span>
                  <span className="flex-none whitespace-nowrap text-xs">{t("learn.picker.coming_soon")}</span>
                </div>
              )}
              {open && topics.length > 0 && (
                <ul className="mb-1 flex flex-col gap-px pl-2">
                  {topics.map((tp) => {
                    const on = tp.id === current.topic;
                    return (
                      <li key={tp.id}>
                        <Link
                          to="/learn/$subject/$topic"
                          params={{ subject: s.id, topic: tp.id }}
                          aria-current={on ? "page" : undefined}
                          className={cn(
                            "flex min-h-8 items-center justify-between rounded-md px-2 hover:bg-accent",
                            on && "bg-accent font-semibold text-primary",
                          )}
                        >
                          <span>{tp.title}</span>
                          {viewed.includes(tp.id) && (
                            <Check
                              role="img"
                              aria-label={t("learn.picker.viewed")}
                              className="size-3.5 flex-none text-verdict-accepted"
                            />
                          )}
                        </Link>
                      </li>
                    );
                  })}
                </ul>
              )}
            </li>
          );
        })}
      </ul>
    </nav>
  );
}
