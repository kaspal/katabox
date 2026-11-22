// Help (expansion M4): the user guide inside the app, by section, in the UI language.
// "Search everything" opens it at a section (`?section=`).
import { useEffect, useMemo } from "react";
import { Link, getRouteApi } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Page } from "@/components/Page";
import { docSections } from "@/lib/docs";
import { cn } from "@/lib/utils";
import { Markdown } from "@/workspace/problem/Markdown";

const route = getRouteApi("/help");

export function Help() {
  const { t, i18n } = useTranslation();
  const { section } = route.useSearch();
  const sections = useMemo(() => docSections(i18n.language), [i18n.language]);
  useEffect(() => {
    if (section) document.getElementById(`help-${section}`)?.scrollIntoView({ block: "start" });
  }, [section, sections]);
  return (
    <Page title={t("help.title")}>
      <div className="flex min-h-0 flex-1 gap-6">
        <nav aria-label={t("help.contents")} className="hidden w-56 shrink-0 overflow-auto lg:block">
          <ul className="space-y-1 text-sm">
            {sections.map((s) => (
              <li key={s.id}>
                <Link
                  to="/help"
                  search={{ section: s.id }}
                  className={cn("block rounded px-2 py-1 hover:bg-accent", section === s.id && "bg-accent font-medium")}
                >
                  {s.title}
                </Link>
              </li>
            ))}
          </ul>
        </nav>
        <div className="min-w-0 max-w-3xl flex-1 space-y-8 overflow-auto pb-12" data-testid="help">
          {sections.map((s) => (
            <section key={s.id} id={`help-${s.id}`} aria-labelledby={`help-h-${s.id}`} className="scroll-mt-4">
              <h2 id={`help-h-${s.id}`} className="mb-2 text-lg font-semibold">
                {s.title}
              </h2>
              <Markdown text={s.body} />
            </section>
          ))}
        </div>
      </div>
    </Page>
  );
}
