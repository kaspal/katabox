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

/** Does `value` match the signature type (`int`, `float`, `string`, `bool`, `T[]`)? */
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
