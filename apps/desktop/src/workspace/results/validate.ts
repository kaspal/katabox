// Custom test validation as you type (ADR 0013): each value must be JSON of the
// parameter's signature type. The backend checks again (and runs the reference solution).
import type { JsonValue } from "@/bindings";
import type { CustomInput } from "@/lib/api";

export type Param = { name: string; type: string };
export type Signature = { name: string; params: Param[]; returns: string };

/** Read `ProblemView.signature` (JSON) into a typed signature, or null if malformed. */
export function parseSignature(v: JsonValue): Signature | null {
  if (typeof v !== "object" || v === null || Array.isArray(v)) return null;
  const params = v.params;
  if (typeof v.name !== "string" || typeof v.returns !== "string" || !Array.isArray(params)) {
    return null;
  }
  const out: Param[] = [];
  for (const p of params) {
    if (typeof p !== "object" || p === null || Array.isArray(p)) return null;
    if (typeof p.name !== "string" || typeof p.type !== "string") return null;
    out.push({ name: p.name, type: p.type });
  }
  return { name: v.name, params: out, returns: v.returns };
}

/** A canonical level-order tree (ADR 0025): safe ints and nulls, the root not null, every
 * slot owned by a node listed before it, and no trailing null. */
export function isLevelOrder(value: unknown): boolean {
  if (!Array.isArray(value)) return false;
  if (!value.every((v) => v === null || (typeof v === "number" && Number.isSafeInteger(v)))) return false;
  if (value.length === 0) return true;
  if (value[0] === null || value[value.length - 1] === null) return false;
  let slots = 1;
  for (let i = 0; i < value.length; i++) {
    if (i >= slots) return false;
    if (value[i] !== null) slots += 2;
  }
  return true;
}

/** The element type under every `[]` (`TreeNode[]` -> `TreeNode`). */
export function elementType(type: string): string {
  let t = type.trim();
  while (t.endsWith("[]")) t = t.slice(0, -2).trim();
  return t;
}

/** Does `value` match the signature type (`int`, `float`, `string`, `bool`, `ListNode`,
 * `TreeNode`, `T[]`)? Lists and trees are JSON arrays (ADR 0025). */
export function matchesType(type: string, value: unknown): boolean {
  const t = type.trim();
  if (t.endsWith("[]")) {
    return Array.isArray(value) && value.every((v) => matchesType(t.slice(0, -2), v));
  }
  switch (t) {
    case "int":
      return typeof value === "number" && Number.isSafeInteger(value);
    case "float":
      return typeof value === "number" && Number.isFinite(value);
    case "string":
      return typeof value === "string";
    case "bool":
      return typeof value === "boolean";
    case "ListNode":
      return Array.isArray(value) && value.every((v) => typeof v === "number" && Number.isSafeInteger(v));
    case "TreeNode":
      return isLevelOrder(value);
    default:
      return false;
  }
}

export type FieldError =
  | { kind: "empty" }
  | { kind: "json"; detail: string }
  | { kind: "type"; expected: string };

export type CustomTestDraft = { id: number; values: Record<string, string> };

export type Validation = { errors: Record<string, FieldError>; input: CustomInput | null };

/** Validate one custom test; `input` is set only when every field is valid. */
export function validateCustom(sig: Signature, values: Record<string, string>): Validation {
  const errors: Record<string, FieldError> = {};
  const input: CustomInput = {};
  for (const p of sig.params) {
    const raw = (values[p.name] ?? "").trim();
    if (raw === "") {
      errors[p.name] = { kind: "empty" };
      continue;
    }
    let parsed: unknown;
    try {
      parsed = JSON.parse(raw);
    } catch (e) {
      errors[p.name] = { kind: "json", detail: e instanceof Error ? e.message : String(e) };
      continue;
    }
    if (!matchesType(p.type, parsed)) {
      errors[p.name] = { kind: "type", expected: p.type };
      continue;
    }
    input[p.name] = parsed as JsonValue;
  }
  return { errors, input: Object.keys(errors).length === 0 ? input : null };
}

/** Field texts for a custom test copied from a public test's input. */
export function valuesFrom(sig: Signature, input: Partial<Record<string, JsonValue>>): Record<string, string> {
  return Object.fromEntries(sig.params.map((p) => [p.name, JSON.stringify(input[p.name] ?? null)]));
}
