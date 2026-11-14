// Word search by backtracking. Mirrors content/learn/backtracking/word-search/sample.{py,ts}:
// try every cell as the first letter; from a matching cell, step to a neighbour (up,
// right, down, left) that matches the next letter and is not already on the path; when
// no neighbour works, step back. Output: [1, then the path's (row, column) pairs] or [0].
import { fail } from "../engine/codecs";
import { type InputError } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"word-search">;
const KEYS = ["word-search.ready", "word-search.step", "word-search.back", "word-search.found", "word-search.missing"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["step", "back"] as const;
const DIRS: [number, number][] = [
  [-1, 0],
  [0, 1],
  [1, 0],
  [0, -1],
];
export const MAX_SIDE = 5;

const split = (input: readonly number[]) => {
  const [rows = 0, cols = 0] = input;
  const rest = input.slice(2);
  const cut = rest.indexOf(-1);
  const letters = rest.slice(0, cut).map((v) => String.fromCharCode(97 + v)).join("");
  const word = rest.slice(cut + 1).map((v) => String.fromCharCode(97 + v)).join("");
  return { rows, cols, letters, word };
};
export const enc = (map: string, word: string) => {
  const rows = map.split("/");
  return [rows.length, rows[0]!.length, ...[...rows.join("")].map((ch) => ch.charCodeAt(0) - 97), -1, ...[...word].map((ch) => ch.charCodeAt(0) - 97)];
};

function trace(input: readonly number[]): Step[] {
  const { rows, cols, letters, word } = split(input);
  const r = new Recorder([]);
  const path: [number, number][] = [];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: [number, number] | null = null, role: Role = "key") => {
    const roles: Record<number, Role> = {};
    for (const [pr, pc] of path) roles[pr * cols + pc] = "sorted";
    if (hot) roles[hot[0] * cols + hot[1]] = role;
    r.snap({
      narration,
      marker,
      board: { rows, cols, cells: [...letters], roles, path: path.map(([pr, pc]) => pr * cols + pc) },
      slots: { layout: "row", label: "word", cells: [...word].map((ch, id) => ({ id, value: id, text: ch })), roles: Object.fromEntries(path.map((_, k) => [k, "sorted" as Role])) },
      vars: { matched: path.length, length: word.length },
    });
  };
  const walk = (row: number, col: number, k: number): boolean => {
    r.comparisons++;
    if (letters[row * cols + col] !== word[k] || path.some(([pr, pc]) => pr === row && pc === col)) return false;
    path.push([row, col]);
    r.moves++;
    snap(say("word-search.step", { letter: word[k]!, row, col, k: k + 1 }), "step", [row, col]);
    if (k === word.length - 1) return true;
    for (const [dr, dc] of DIRS) {
      const nr = row + dr;
      const nc = col + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && walk(nr, nc, k + 1)) return true;
    }
    path.pop();
    r.writes++;
    snap(say("word-search.back", { letter: word[k]!, row, col }), "back", [row, col], "out");
    return false;
  };

  snap(say("word-search.ready", { word, rows, cols }), null);
  for (let row = 0; row < rows; row++) {
    for (let col = 0; col < cols; col++) {
      if (walk(row, col, 0)) {
        snap(say("word-search.found", { word, tries: r.comparisons }), null);
        r.steps[r.steps.length - 1]!.result = [1, ...path.flat()];
        return r.steps;
      }
    }
  }
  snap(say("word-search.missing", { word }), null);
  r.steps[r.steps.length - 1]!.result = [0];
  return r.steps;
}

const check = (input: readonly number[]): InputError | null => {
  const { rows, cols, letters, word } = split(input);
  if (rows < 1 || rows > MAX_SIDE || cols < 1 || cols > MAX_SIDE) return { code: "custom", key: "grid_size", params: { rows: MAX_SIDE, cols: MAX_SIDE } };
  if (letters.length !== rows * cols) return { code: "custom", key: "grid_ragged" };
  if (word.length < 1 || word.length > 8) return { code: "custom", key: "word_length", params: { field: "word", min: 1, max: 8 } };
  return null;
};

export const wordSearch: TracerDef = {
  id: "word-search",
  subject: "backtracking",
  scene: "board",
  limits: { min_len: 5, max_len: 36, min: -1, max: 25 },
  input: {
    kind: "codec",
    codec: {
      fields: ["map", "word"],
      format: (input) => {
        const { rows, cols, letters, word } = split(input);
        return [Array.from({ length: rows }, (_, k) => letters.slice(k * cols, (k + 1) * cols)).join("/"), word];
      },
      parse: (texts) => {
        const rows = (texts[0] ?? "").toLowerCase().split(/[/\n]+/).map((s) => s.trim()).filter((s) => s !== "");
        const word = (texts[1] ?? "").trim().toLowerCase();
        if (rows.some((row) => !/^[a-z]+$/.test(row)) || !/^[a-z]*$/.test(word)) return fail("letters_only", { field: "map" });
        if (rows.some((row) => row.length !== rows[0]!.length)) return fail("grid_ragged");
        const values = enc(rows.join("/"), word);
        const e = check(values);
        return e ? { error: e } : { values };
      },
      check,
    },
  },
  presets: {
    "winding-word": () => enc("wqzk/arym/tels/bnow", "water"),
    random: (next) => {
      const side = 3 + Math.floor(next() * 3);
      const letters = Array.from({ length: side * side }, () => "abcde"[Math.floor(next() * 5)]!).join("");
      const word = Array.from({ length: 3 + Math.floor(next() * 3) }, () => "abcde"[Math.floor(next() * 5)]!).join("");
      return enc(Array.from({ length: side }, (_, k) => letters.slice(k * side, (k + 1) * side)).join("/"), word);
    },
    "false-start": () => enc("caat/aacr/tcar/rtca", "cart"),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "moves", label: "steps_taken" },
    { slot: "writes", label: "undone" },
  ],
  legend: ["key", "sorted", "out"],
  roleLabels: { key: "current", sorted: "on_the_path", out: "removed" },
  vars: ["matched", "length"],
  trace,
};
