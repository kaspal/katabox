// Codecs for the DP and greedy topics of Phase 8 (ADR 0029): intervals, a word and a
// dictionary, a blocked-cell map, a distance matrix and items. Each stores one flat
// integer list the Learn driver reads with its usual helpers (kb_chunks, kb_text).
import { fail } from "./codecs";
import { tokens, type InputError } from "./inputs";
import type { FieldKey, InputCodec } from "./types";

const err = (key: string, params?: Record<string, string | number>): InputError => ({ code: "custom", key, params });

/** Groups of `size` numbers separated by commas ("1 4, 2 5"), stored flat. */
function groups(text: string, size: number, what: string): { values: number[] } | { error: InputError } {
  const out: number[] = [];
  for (const chunk of text.split(/[,;\n]+/).map((s) => s.trim()).filter((s) => s !== "")) {
    const t = tokens(chunk);
    if ("error" in t) return t;
    if (t.values.length !== size) return fail(what, { item: chunk });
    out.push(...t.values);
  }
  return { values: out };
}

/** Intervals [start, end) as "start end" pairs: stored [s, e, s, e, …]. */
export function intervalsCodec(o: { min: number; max: number; count: [number, number] }): InputCodec {
  const check = (input: readonly number[]): InputError | null => {
    if (input.length % 2 !== 0) return err("interval_pair", { item: "" });
    const n = input.length / 2;
    if (n < o.count[0] || n > o.count[1]) return err("interval_count", { min: o.count[0], max: o.count[1] });
    for (let k = 0; k < n; k++) {
      const s = input[2 * k]!;
      const e = input[2 * k + 1]!;
      if (s < o.min || e > o.max || s >= e) return err("interval_range", { min: o.min, max: o.max });
    }
    return null;
  };
  return {
    fields: ["intervals"],
    format: (input) => [Array.from({ length: input.length / 2 }, (_, k) => `${input[2 * k]} ${input[2 * k + 1]}`).join(", ")],
    parse: (texts) => {
      const r = groups(texts[0] ?? "", 2, "interval_pair");
      if ("error" in r) return r;
      const e = check(r.values);
      return e ? { error: e } : r;
    },
    check,
  };
}

/** Capacity and items as "weight value" pairs: stored [capacity, w, v, w, v, …]. */
export function itemsCodec(o: { capacity: number; count: number; weight: number }): InputCodec {
  const check = (input: readonly number[]): InputError | null => {
    if (input[0]! < 1 || input[0]! > o.capacity) return err("capacity_range", { max: o.capacity });
    if ((input.length - 1) % 2 !== 0) return err("item_pair", { item: "" });
    const n = (input.length - 1) / 2;
    if (n < 1 || n > o.count) return err("item_count", { min: 1, max: o.count });
    for (let k = 0; k < n; k++) if (input[1 + 2 * k]! > o.weight) return err("weight_range", { min: 1, max: o.weight });
    return null;
  };
  return {
    fields: ["capacity", "items"],
    format: (input) => [String(input[0] ?? ""), Array.from({ length: (input.length - 1) / 2 }, (_, k) => `${input[1 + 2 * k]} ${input[2 + 2 * k]}`).join(", ")],
    parse: (texts) => {
      const cap = tokens(texts[0] ?? "");
      if ("error" in cap) return cap;
      if (cap.values.length !== 1) return fail("one_number", { field: "capacity" });
      const r = groups(texts[1] ?? "", 2, "item_pair");
      if ("error" in r) return r;
      const values = [cap.values[0]!, ...r.values];
      const e = check(values);
      return e ? { error: e } : { values };
    },
    check,
  };
}

/** A map of open (.) and blocked (#) cells: stored [rows, cols, cell…]. */
export function blocksCodec(o: { rows: number; cols: number }): InputCodec {
  const check = (input: readonly number[]): InputError | null => {
    const [rows = 0, cols = 0] = input;
    if (rows < 1 || rows > o.rows || cols < 1 || cols > o.cols || rows * cols < 2) return err("grid_size", { rows: o.rows, cols: o.cols });
    if (input.length !== 2 + rows * cols) return err("grid_ragged");
    if (input.slice(2).some((v) => v !== 0 && v !== 1)) return err("grid_no_weights", { ch: "?" });
    return null;
  };
  return {
    fields: ["map"],
    format: (input) => {
      const [rows = 0, cols = 0] = input;
      return [Array.from({ length: rows }, (_, r) => input.slice(2 + r * cols, 2 + (r + 1) * cols).map((v) => (v ? "#" : ".")).join("")).join("/")];
    },
    parse: (texts) => {
      const lines = (texts[0] ?? "").split(/[/\n]+/).map((l) => l.trim()).filter((l) => l !== "");
      if (lines.length < 1 || lines.length > o.rows || lines.some((l) => l.length > o.cols)) return fail("grid_size", { rows: o.rows, cols: o.cols });
      if (lines.some((l) => l.length !== lines[0]!.length)) return fail("grid_ragged");
      const bad = lines.join("").match(/[^.#]/);
      if (bad) return fail("grid_no_weights", { ch: bad[0] });
      const values = [lines.length, lines[0]!.length, ...[...lines.join("")].map((ch) => (ch === "#" ? 1 : 0))];
      const e = check(values);
      return e ? { error: e } : { values };
    },
    check,
  };
}

/** A symmetric distance matrix, typed as rows separated by commas: stored [n, d00, d01, …]. */
export function matrixCodec(o: { min: number; max: number; weight: number }): InputCodec {
  const check = (input: readonly number[]): InputError | null => {
    const n = input[0] ?? 0;
    if (n < o.min || n > o.max) return err("node_count", { min: o.min, max: o.max });
    if (input.length !== 1 + n * n) return err("matrix_shape", { n });
    const d = (i: number, j: number) => input[1 + i * n + j]!;
    for (let i = 0; i < n; i++)
      for (let j = 0; j < n; j++) {
        if (i === j && d(i, j) !== 0) return err("matrix_diagonal");
        if (d(i, j) !== d(j, i)) return err("matrix_symmetric", { i, j });
        if (i !== j && (d(i, j) < 1 || d(i, j) > o.weight)) return err("weight_range", { min: 1, max: o.weight });
      }
    return null;
  };
  return {
    fields: ["distances"],
    format: (input) => {
      const n = input[0] ?? 0;
      return [Array.from({ length: n }, (_, i) => input.slice(1 + i * n, 1 + (i + 1) * n).join(" ")).join(", ")];
    },
    parse: (texts) => {
      const rows = (texts[0] ?? "").split(/[,;\n]+/).map((s) => s.trim()).filter((s) => s !== "");
      const values: number[] = [rows.length];
      for (const row of rows) {
        const t = tokens(row);
        if ("error" in t) return t;
        if (t.values.length !== rows.length) return fail("matrix_shape", { n: rows.length });
        values.push(...t.values);
      }
      const e = check(values);
      return e ? { error: e } : { values };
    },
    check,
  };
}

/** A text and a list of words (letters a-z): stored text, -1, word, -1, word, … */
export function textWordsCodec(o: { maxText: number; maxWords: number; maxWord: number }, fields: [FieldKey, FieldKey] = ["text", "words"]): InputCodec {
  const decode = (input: readonly number[]) => {
    const words: string[] = [""];
    for (const v of input) {
      if (v === -1) words.push("");
      else words[words.length - 1] += String.fromCharCode(97 + v);
    }
    return words;
  };
  const check = (input: readonly number[]): InputError | null => {
    const [text = "", ...words] = decode(input);
    if (text.length < 1 || text.length > o.maxText) return err("word_length", { field: fields[0], min: 1, max: o.maxText });
    if (words.length < 1 || words.length > o.maxWords) return err("word_count", { min: 1, max: o.maxWords });
    if (words.some((w) => w.length < 1 || w.length > o.maxWord)) return err("word_length", { field: fields[1], min: 1, max: o.maxWord });
    return null;
  };
  return {
    fields,
    format: (input) => {
      const [text = "", ...words] = decode(input);
      return [text, words.join(" ")];
    },
    parse: (texts) => {
      const text = (texts[0] ?? "").trim().toLowerCase();
      const words = (texts[1] ?? "").toLowerCase().split(/[\s,;]+/).filter((w) => w !== "");
      for (const [w, field] of [[text, fields[0]], ...words.map((x) => [x, fields[1]] as const)] as const) {
        if (!/^[a-z]*$/.test(w)) return fail("letters_only", { field });
      }
      const values = [text, ...words].flatMap((w, k) => [...(k > 0 ? [-1] : []), ...[...w].map((c) => c.charCodeAt(0) - 97)]);
      const e = check(values);
      return e ? { error: e } : { values };
    },
    check,
  };
}

/** Database rows (the Databases subject): one or two tables of "a b" rows, comma
 * separated. One table is stored [a, b, a, b, …]; two are stored [n_first, …first rows,
 * …second rows]. Both columns are small whole numbers within their ranges. */
export function rowsCodec(o: {
  fields: readonly [FieldKey] | readonly [FieldKey, FieldKey];
  a: [number, number];
  b: [number, number];
  count: [number, number];
}): InputCodec {
  const two = o.fields.length === 2;
  const split = (input: readonly number[]): number[][] =>
    two ? [input.slice(1, 1 + 2 * input[0]!), input.slice(1 + 2 * input[0]!)] : [[...input]];
  const check = (input: readonly number[]): InputError | null => {
    if (two && (input.length < 1 || input[0]! < 0 || 1 + 2 * input[0]! > input.length)) return err("row_pair", { item: "" });
    for (const [k, rows] of split(input).entries()) {
      if (rows.length % 2 !== 0) return err("row_pair", { item: "" });
      const n = rows.length / 2;
      if (n < o.count[0] || n > o.count[1]) return err("row_count", { field: o.fields[k]!, min: o.count[0], max: o.count[1] });
      for (let r = 0; r < n; r++) {
        const a = rows[2 * r]!;
        const b = rows[2 * r + 1]!;
        if (a < o.a[0] || a > o.a[1] || b < o.b[0] || b > o.b[1]) return err("row_range", { amin: o.a[0], amax: o.a[1], bmin: o.b[0], bmax: o.b[1] });
      }
    }
    return null;
  };
  const show = (rows: number[]) => Array.from({ length: rows.length / 2 }, (_, k) => `${rows[2 * k]} ${rows[2 * k + 1]}`).join(", ");
  return {
    fields: o.fields,
    format: (input) => split(input).map(show),
    parse: (texts) => {
      const tables: number[][] = [];
      for (const text of texts.slice(0, o.fields.length)) {
        const r = groups(text ?? "", 2, "row_pair");
        if ("error" in r) return r;
        tables.push(r.values);
      }
      const values = two ? [tables[0]!.length / 2, ...tables[0]!, ...(tables[1] ?? [])] : (tables[0] ?? []);
      const e = check(values);
      return e ? { error: e } : { values };
    },
    check,
  };
}

/** Each employee's boss (-1 for none), employees numbered from 0: stored as the list. The
 * chain of bosses never loops back. */
export function bossesCodec(o: { count: [number, number] }): InputCodec {
  const check = (input: readonly number[]): InputError | null => {
    const n = input.length;
    if (n < o.count[0] || n > o.count[1]) return err("boss_count", { min: o.count[0], max: o.count[1] });
    for (let i = 0; i < n; i++) {
      const b = input[i]!;
      if (b !== -1 && (b < 0 || b >= n || b === i)) return err("boss_range", { max: n - 1 });
    }
    for (let i = 0; i < n; i++) {
      let x = i;
      for (let steps = 0; x !== -1; steps++) {
        if (steps > n) return err("boss_cycle");
        x = input[x]!;
      }
    }
    return null;
  };
  return {
    fields: ["bosses"],
    format: (input) => [input.join(" ")],
    parse: (texts) => {
      const r = tokens(texts[0] ?? "");
      if ("error" in r) return r;
      const e = check(r.values);
      return e ? { error: e } : r;
    },
    check,
  };
}
