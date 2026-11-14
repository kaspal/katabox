// Input codecs (ADR 0029, Phases 2-6): structured inputs typed as text in "your own" and
// stored as one flat integer list everywhere else (presets, cases.json, the sample
// driver). The Learn driver has the matching decoders (`kb_ops`, `kb_text`).
import { tokens, type InputError } from "./inputs";
import type { FieldKey, InputCodec } from "./types";

export const fail = (key: string, params?: Record<string, string | number>): { error: InputError } => ({
  error: { code: "custom", key, params },
});

export type OpSpec = { name: string; args: number };

/** Operations such as `push 3, pop, peek`: each op is stored as its 1-based code followed
 * by `width` arguments (unused ones are 0). `header` fields come first, one integer each
 * (a capacity, a size). */
export function opsCodec(ops: readonly OpSpec[], header: readonly FieldKey[] = []): InputCodec & { width: number } {
  const width = Math.max(0, ...ops.map((o) => o.args));
  const h = header.length;
  return {
    width,
    fields: [...header, "ops"],
    format: (input) => {
      const parts: string[] = [];
      for (let k = h; k < input.length; k += width + 1) {
        const op = ops[input[k]! - 1];
        if (!op) break;
        parts.push([op.name, ...input.slice(k + 1, k + 1 + op.args)].join(" "));
      }
      return [...input.slice(0, h).map(String), parts.join(", ")];
    },
    parse: (texts) => {
      const out: number[] = [];
      for (let k = 0; k < h; k++) {
        const r = tokens(texts[k] ?? "");
        if ("error" in r) return r;
        if (r.values.length !== 1) return fail("one_number", { field: header[k]! });
        out.push(r.values[0]!);
      }
      const text = (texts[h] ?? "").trim();
      if (text === "") return fail("no_ops");
      for (const chunk of text.split(/[,;\n]+/).map((c) => c.trim()).filter((c) => c !== "")) {
        const [name, ...rest] = chunk.split(/\s+/);
        const code = ops.findIndex((o) => o.name === name!.toLowerCase());
        if (code < 0) return fail("unknown_op", { op: name!, ops: ops.map((o) => o.name).join(", ") });
        const op = ops[code]!;
        if (rest.length !== op.args) return fail("op_args", { op: op.name, count: op.args });
        const args: number[] = [];
        for (const a of rest) {
          if (!/^-?\d+$/.test(a)) return { error: { code: "nan", token: a } };
          args.push(Number(a));
        }
        out.push(code + 1, ...args, ...new Array<number>(width - args.length).fill(0));
      }
      return { values: out };
    },
    check: (input) => {
      if ((input.length - h) % (width + 1) !== 0 || input.length === h) return { code: "custom", key: "no_ops" };
      for (let k = h; k < input.length; k += width + 1) {
        if (!ops[input[k]! - 1]) return { code: "custom", key: "unknown_op", params: { op: String(input[k]), ops: ops.map((o) => o.name).join(", ") } };
      }
      return null;
    },
  };
}

/** Decode stored operations (the tracer's view of `kb_ops`). */
export function decodeOps(input: readonly number[], ops: readonly OpSpec[], width: number, header = 0) {
  const out: { name: string; code: number; args: number[] }[] = [];
  for (let k = header; k < input.length; k += width + 1) {
    const op = ops[input[k]! - 1]!;
    out.push({ name: op.name, code: input[k]!, args: input.slice(k + 1, k + 1 + op.args) });
  }
  return out;
}

/** Lowercase words a-z, one per field, stored as letter codes 0-25 separated by -1. */
export function textCodec(fields: readonly FieldKey[], opts: { minLen?: number; maxLen?: number } = {}): InputCodec {
  const minLen = opts.minLen ?? 1;
  const maxLen = opts.maxLen ?? 16;
  return {
    fields,
    format: (input) => decodeText(input),
    parse: (texts) => {
      const out: number[] = [];
      for (let k = 0; k < fields.length; k++) {
        const word = (texts[k] ?? "").trim().toLowerCase();
        if (!/^[a-z]*$/.test(word)) return fail("letters_only", { field: fields[k]! });
        if (word.length < minLen || word.length > maxLen) {
          return fail("word_length", { field: fields[k]!, min: minLen, max: maxLen });
        }
        if (k > 0) out.push(-1);
        for (const c of word) out.push(c.charCodeAt(0) - 97);
      }
      return { values: out };
    },
    check: (input) => {
      const words = decodeText(input);
      if (words.length !== fields.length) return { code: "custom", key: "size" };
      for (const w of words) if (w.length < minLen || w.length > maxLen) return { code: "custom", key: "size" };
      return null;
    },
  };
}

/** The words of a text input (the tracer's view of `kb_text`). */
export function decodeText(input: readonly number[]): string[] {
  const words: string[] = [""];
  for (const v of input) {
    if (v === -1) words.push("");
    else words[words.length - 1] += String.fromCharCode(97 + v);
  }
  return words;
}

/** Encode words as a text input (presets). */
export function encodeText(words: readonly string[]): number[] {
  return words.flatMap((w, k) => [...(k > 0 ? [-1] : []), ...[...w].map((c) => c.charCodeAt(0) - 97)]);
}

/** A list of values followed by operations: stored as `[n, v1..vn, ops...]` (the sample
 * splits it with `a[1:1 + a[0]]`). `check` validates the list's size and range; op
 * arguments are checked by the tracer's own `check`. */
export function listOpsCodec(
  list: FieldKey,
  ops: readonly OpSpec[],
  bounds: { minLen: number; maxLen: number; min: number; max: number },
): InputCodec & { width: number; split: (input: readonly number[]) => { values: number[]; ops: ReturnType<typeof decodeOps> } } {
  const inner = opsCodec(ops);
  const sizeError = () => fail("list_size", { field: list, min: bounds.minLen, max: bounds.maxLen, lo: bounds.min, hi: bounds.max });
  return {
    width: inner.width,
    fields: [list, "ops"],
    format: (input) => {
      const n = input[0] ?? 0;
      return [input.slice(1, 1 + n).join(" "), inner.format(input.slice(1 + n))[0]!];
    },
    parse: (texts) => {
      const r = tokens(texts[0] ?? "");
      if ("error" in r) return r;
      if (r.values.length < bounds.minLen || r.values.length > bounds.maxLen) return sizeError();
      if (r.values.some((v) => v < bounds.min || v > bounds.max)) return sizeError();
      const o = inner.parse([texts[1] ?? ""]);
      if ("error" in o) return o;
      return { values: [r.values.length, ...r.values, ...o.values] };
    },
    check: (input) => {
      const n = input[0] ?? 0;
      if (n < bounds.minLen || n > bounds.maxLen || input.length < 1 + n) return sizeError().error;
      if (input.slice(1, 1 + n).some((v) => v < bounds.min || v > bounds.max)) return sizeError().error;
      return inner.check!(input.slice(1 + n));
    },
    split: (input) => {
      const n = input[0]!;
      return { values: input.slice(1, 1 + n), ops: decodeOps(input.slice(1 + n), ops, inner.width) };
    },
  };
}
