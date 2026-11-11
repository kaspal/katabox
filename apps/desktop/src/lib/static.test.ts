// Checks that need no rendering: i18n key parity, error-code mapping, token contrast.
import { describe, expect, it } from "vitest";
import i18next from "i18next";
import en from "@/i18n/locales/en.json";
import es from "@/i18n/locales/es.json";
import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { describeError, describeMessage } from "@/lib/errors";
import { resources } from "@/i18n";

function keys(obj: object, prefix = ""): string[] {
  return Object.entries(obj).flatMap(([k, v]) =>
    typeof v === "object" && v !== null ? keys(v, `${prefix}${k}.`) : [`${prefix}${k}`],
  );
}

describe("i18n resources", () => {
  it("English and Spanish have the same keys", () => {
    expect(keys(es).sort()).toEqual(keys(en).sort());
  });

  it("no translation is empty", () => {
    for (const [lang, res] of [["en", en], ["es", es]] as const) {
      const empty = keys(res).filter((k) => {
        const v = k.split(".").reduce<unknown>((o, p) => (o as Record<string, unknown>)[p], res);
        return typeof v !== "string" || v.trim() === "";
      });
      expect(empty, lang).toEqual([]);
    }
  });
});

describe("generated shadcn components", () => {
  it("import cn from our utils, never the unrelated npm package \"cn\"", () => {
    const dir = path.resolve(process.cwd(), "src/components/ui");
    const bad = readdirSync(dir).filter((f) => readFileSync(path.join(dir, f), "utf8").includes('from "cn"'));
    expect(bad).toEqual([]);
  });
});

describe("error and message codes", () => {
  const i18n = i18next.createInstance();
  void i18n.init({ resources, lng: "es", fallbackLng: "en", initAsync: false });

  it("maps known codes to the UI language and keeps the detail", () => {
    const d = describeError(i18n, { code: "judge_busy", message: "a Run is in progress" });
    expect(d).toEqual({ summary: "Ya hay una ejecución en curso.", detail: "a Run is in progress" });
  });

  it("falls back to the English text for unknown codes", () => {
    expect(describeError(i18n, { code: "brand_new", message: "Something new" })).toEqual({
      summary: "Something new",
    });
    expect(describeMessage(i18n, { code: "exception", text: "Traceback…" })).toBe(
      "La función lanzó un error.",
    );
    expect(describeMessage(i18n, { code: "brand_new" as never, text: "English text" })).toBe(
      "English text",
    );
  });

  it("treats anything else as a generic error", () => {
    expect(describeError(i18n, new Error("boom"))).toEqual({
      summary: "Algo salió mal.",
      detail: "boom",
    });
  });
});

// --- WCAG contrast of the design tokens -----------------------------------------------

// Vitest runs from apps/desktop (jsdom has no file: import.meta.url).
const tokensCss = readFileSync(path.resolve(process.cwd(), "src/styles/tokens.css"), "utf8");

function block(selector: string): Record<string, string> {
  const start = tokensCss.indexOf(`${selector} {`);
  const body = tokensCss.slice(start, tokensCss.indexOf("\n}", start));
  return Object.fromEntries(
    [...body.matchAll(/--([\w-]+):\s*(#[0-9a-fA-F]{6})/g)].map((m) => [m[1], m[2]]),
  );
}

function luminance(hex: string): number {
  const [r, g, b] = [1, 3, 5].map((i) => {
    const c = parseInt(hex.slice(i, i + 2), 16) / 255;
    return c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

function contrast(a: string, b: string): number {
  const [x, y] = [luminance(a), luminance(b)].sort((p, q) => q - p);
  return (x + 0.05) / (y + 0.05);
}

const TEXT_ON = ["background", "card", "muted", "accent", "popover"];
const SEMANTIC = [
  "foreground",
  "muted-foreground",
  "primary",
  "verdict-accepted",
  "verdict-wrong",
  "verdict-error",
  "verdict-limit",
  "difficulty-easy",
  "difficulty-medium",
  "difficulty-hard",
  "status-solved",
  "status-attempted",
  "destructive",
  "syntax-keyword",
  "syntax-string",
  "syntax-number",
  "syntax-comment",
  "syntax-function",
  "syntax-type",
  "syntax-variable",
  "syntax-operator",
  "syntax-invalid",
];

describe.each([
  ["light", block(":root")],
  ["dark", { ...block(":root"), ...block(".dark") }],
])("%s theme contrast (WCAG AA)", (_, t) => {
  it.each(SEMANTIC)("%s text is ≥ 4.5:1 on every surface", (fg) => {
    for (const bg of TEXT_ON) {
      expect(contrast(t[fg], t[bg]), `${fg} on ${bg}`).toBeGreaterThanOrEqual(4.5);
    }
  });

  it("filled colors keep their text readable", () => {
    expect(contrast(t["primary-foreground"], t.primary)).toBeGreaterThanOrEqual(4.5);
    expect(contrast(t["destructive-foreground"], t.destructive)).toBeGreaterThanOrEqual(4.5);
    expect(contrast(t["dev-badge-foreground"], t["dev-badge"])).toBeGreaterThanOrEqual(4.5);
  });

  it("the focus ring and inputs stand out (≥ 3:1)", () => {
    for (const bg of ["background", "card"]) {
      expect(contrast(t.ring, t[bg]), `ring on ${bg}`).toBeGreaterThanOrEqual(3);
    }
  });
});
