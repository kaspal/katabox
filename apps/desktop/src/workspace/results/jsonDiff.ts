// A small JSON diff for "expected vs actual": which top-level elements (arrays) or keys
// (objects) differ. Enough to point at the wrong element without a diff library.
import type { JsonValue } from "@/bindings";

export type DiffPart = { text: string; changed: boolean };

const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);

/** `value` split into parts, marking elements that differ from `other`. */
export function diffParts(value: JsonValue | null | undefined, other: JsonValue | null | undefined): DiffPart[] {
  if (value === undefined) return [{ text: "—", changed: true }];
  if (Array.isArray(value) && Array.isArray(other)) {
    const parts: DiffPart[] = [{ text: "[", changed: false }];
    value.forEach((v, i) => {
      if (i > 0) parts.push({ text: ", ", changed: false });
      parts.push({ text: JSON.stringify(v), changed: !same(v, other[i]) });
    });
    if (value.length < other.length) parts.push({ text: " …", changed: true });
    parts.push({ text: "]", changed: false });
    return parts;
  }
  if (isObject(value) && isObject(other)) {
    const keys = Object.keys(value);
    const parts: DiffPart[] = [{ text: "{", changed: false }];
    keys.forEach((k, i) => {
      if (i > 0) parts.push({ text: ", ", changed: false });
      parts.push({
        text: `${JSON.stringify(k)}: ${JSON.stringify(value[k])}`,
        changed: !same(value[k], other[k]),
      });
    });
    parts.push({ text: "}", changed: false });
    return parts;
  }
  return [{ text: JSON.stringify(value), changed: !same(value, other) }];
}

function isObject(v: unknown): v is Record<string, JsonValue> {
  return typeof v === "object" && v !== null && !Array.isArray(v);
}
