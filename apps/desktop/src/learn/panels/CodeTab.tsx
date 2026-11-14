// The Code tab (ADR 0029): the sample in Python or TypeScript with the line behind the
// current step highlighted, and the variables grid. Marker comments are stripped.
import { useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import type { LearnSample } from "@/bindings";
import { highlightWith, type Highlighted } from "@/workspace/editor/syntax";
import { loadFenceParser } from "@/workspace/editor/languages";
import { cn } from "@/lib/utils";
import { parseSample } from "../engine/markers";
import type { Step, TracerDef, VarValue } from "../engine/types";

export type SampleLang = "python" | "typescript";
export const SAMPLE_LANGS: SampleLang[] = ["python", "typescript"];

/** Highlighted runs split into lines (plain text until the grammar loads). */
function useLines(code: string, lang: SampleLang): Highlighted[] | null {
  const [lines, setLines] = useState<Highlighted[] | null>(null);
  useEffect(() => {
    let live = true;
    setLines(null);
    void loadFenceParser(lang).then((parser) => {
      if (!live || !parser) return;
      const out: Highlighted[] = [[]];
      for (const run of highlightWith(parser, code)) {
        if (run.text === "\n") out.push([]);
        else out[out.length - 1]!.push(run);
      }
      setLines(out);
    });
    return () => {
      live = false;
    };
  }, [code, lang]);
  return lines;
}

export function formatVar(v: VarValue | undefined, lang: SampleLang, none: string): string {
  if (v === null || v === undefined) return none;
  if (typeof v === "boolean") return lang === "python" ? (v ? "True" : "False") : String(v);
  return String(v);
}

export function CodeTab({
  samples,
  step,
  def,
  lang,
  onLang,
}: {
  samples: LearnSample[];
  step: Step;
  def: TracerDef;
  lang: SampleLang;
  onLang: (l: SampleLang) => void;
}) {
  const { t } = useTranslation();
  const source = samples.find((s) => s.language === lang)?.source ?? "";
  const view = useMemo(() => parseSample(source), [source]);
  const highlighted = useLines(view.lines.join("\n"), lang);
  const current = step.marker ? view.lineOf[step.marker] : undefined;
  const currentRef = useRef<HTMLDivElement | null>(null);
  useEffect(() => {
    currentRef.current?.scrollIntoView?.({ block: "nearest" });
  }, [current]);

  return (
    <div className="flex flex-col gap-3 p-4">
      <div className="flex items-center justify-between gap-2">
        <div role="group" aria-label={t("learn.code.language")} className="flex rounded-lg bg-muted p-0.5">
          {SAMPLE_LANGS.map((l) => (
            <button
              key={l}
              type="button"
              aria-pressed={lang === l}
              onClick={() => onLang(l)}
              className={cn(
                "h-7 rounded-md px-3 text-sm font-medium text-muted-foreground transition-colors",
                lang === l && "bg-card text-foreground shadow-sm",
              )}
            >
              {t(`languages.${l}`)}
            </button>
          ))}
        </div>
      </div>
      <div
        className="overflow-x-auto rounded-lg border bg-muted/30 py-2 [font-variant-ligatures:none]"
        data-testid="sample-code"
      >
        {view.lines.map((text, k) => {
          const on = k + 1 === current;
          return (
            <div
              key={k}
              ref={on ? currentRef : undefined}
              aria-current={on ? "step" : undefined}
              className={cn(
                "flex font-mono text-[13px] leading-[22px]",
                on && "bg-viz-code-line shadow-[inset_3px_0_0_var(--primary)]",
              )}
              data-line={k + 1}
            >
              <span className="w-9 flex-none select-none pr-3 text-right text-muted-foreground" aria-hidden>
                {k + 1}
              </span>
              {/* Long lines wrap instead of scrolling sideways. */}
              <span className="min-w-0 flex-1 whitespace-pre-wrap [overflow-wrap:anywhere]">
                {on && <span className="sr-only">{t("learn.code.current")}: </span>}
                {highlighted?.[k]
                  ? highlighted[k]!.map((r, i) =>
                      r.className ? (
                        <span key={i} className={r.className}>
                          {r.text}
                        </span>
                      ) : (
                        r.text
                      ),
                    )
                  : text || " "}
              </span>
            </div>
          );
        })}
      </div>
      <div>
        <div className="mb-2 text-xs font-semibold uppercase tracking-wide text-muted-foreground">
          {t("learn.code.variables")}
        </div>
        {/* Up to five variables share one row, even at the narrowest panel (values truncate,
            the full value is in the title); more wrap onto a second row. */}
        <dl
          className="grid gap-1"
          style={{ gridTemplateColumns: `repeat(${Math.min(Math.max(def.vars.length, 1), 5)}, minmax(0, 1fr))` }}
          data-testid="variables"
        >
          {def.vars.map((name) => {
            const value = formatVar(step.vars[name], lang, t("learn.code.none"));
            return (
              <div key={name} className="min-w-0 rounded-md border px-1.5 py-1 font-mono" title={`${name} = ${value}`}>
                <dt className="truncate text-[11px] text-muted-foreground">{name}</dt>
                <dd className="truncate text-[14px] font-medium">{value}</dd>
              </div>
            );
          })}
        </dl>
      </div>
      <p className="text-xs leading-relaxed text-muted-foreground">{t("learn.code.note")}</p>
    </div>
  );
}
