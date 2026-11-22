// Narration rendering (ADR 0029): every step of every topic, on every preset and the
// random cases, rendered in English, Spanish and Portuguese with the app's real i18next instance.
// Key parity can't catch a param that i18next reads as an option (`replace` blanks every
// placeholder, `count` picks a plural form), so this renders the text and inspects it.
import { describe, expect, it } from "vitest";
import i18n, { initI18n } from "@/i18n";
import en from "@/i18n/locales/en.json";
import es from "@/i18n/locales/es.json";
import ptBR from "@/i18n/locales/pt-BR.json";
import { caseInputs } from "../engine/cases";
import { Recorder } from "../engine/recorder";
import { RESERVED_NARRATION_PARAMS, type NarrationParams, type TracerDef } from "../engine/types";
import { TRACERS } from ".";

initI18n("en");

const LOCALES = { en, es, "pt-BR": ptBR } as const;
/** Fragments an empty interpolation leaves behind, unless the template has them itself. */
const EMPTY_SIGNS = ["()", '""', "= .", "  ", " ,", " ."];

function template(lang: keyof typeof LOCALES, key: string): string {
  const v = key.split(".").reduce<unknown>((o, k) => (o as Record<string, unknown> | undefined)?.[k], LOCALES[lang].learn.narration);
  return typeof v === "string" ? v : "";
}
const placeholders = (text: string) => [...text.matchAll(/\{\{\s*(\w+)/g)].map((m) => m[1]!);

/** Every problem found rendering one topic's narration, as readable lines. */
function problems(def: TracerDef): string[] {
  const out: string[] = [];
  const seen = new Set<string>();
  for (const c of caseInputs(def)) {
    for (const step of def.trace(c.input)) {
      const { key, params = {} } = step.narration;
      for (const lang of ["en", "es", "pt-BR"] as const) {
        const tpl = template(lang, key);
        const t = i18n.getFixedT(lang);
        const text = t(`learn.narration.${key}` as "learn.title", params as never) as unknown as string;
        const fail = (why: string) => {
          const line = `${lang} ${key} [${c.name}]: ${why}: ${text}`;
          if (!seen.has(`${lang} ${key} ${why}`)) out.push(line);
          seen.add(`${lang} ${key} ${why}`);
        };
        if (!tpl) fail("no template");
        for (const name of RESERVED_NARRATION_PARAMS) if (Object.hasOwn(params, name)) fail(`reserved param ${name}`);
        if (text.includes("{{")) fail("leftover {{");
        for (const p of placeholders(tpl)) {
          const v = (params as Record<string, unknown>)[p];
          if (v === undefined || v === null || String(v) === "") fail(`empty {{${p}}}`);
        }
        for (const sign of EMPTY_SIGNS) if (text.includes(sign) && !tpl.includes(sign)) fail(`"${sign}"`);
      }
    }
  }
  return out;
}

describe("narration renders in every UI language", () => {
  it.each(Object.values(TRACERS))("$id", (def) => {
    expect(problems(def)).toEqual([]);
  });

  it("refuses i18next option names as params, in the type and at run time", () => {
    // @ts-expect-error: `replace` is i18next's interpolation map, not a value
    const bad: NarrationParams<number> = { replace: 1 };
    const r = new Recorder([1]);
    expect(() => r.snap({ narration: { key: "stack.done", params: bad }, marker: null })).toThrow(/i18next option/);
    expect(() => r.snap({ narration: { key: "stack.done", params: { num: 1 } }, marker: null })).not.toThrow();
  });
});
