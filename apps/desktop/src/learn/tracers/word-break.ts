// Word break: can a text be cut into dictionary words? Mirrors
// content/learn/dynamic-programming/word-break/sample.{py,ts}: ok[end] is true when the
// first `end` letters can be cut; it is, when some word ends exactly at `end` and ok was
// true where that word starts. Words are tried in the order given; the first that fits
// settles the cell. Output: [1] or [0].
import { encodeText } from "../engine/codecs";
import { textWordsCodec } from "../engine/shapes";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"word-break">;
const KEYS = ["word-break.ready", "word-break.end", "word-break.fits", "word-break.none", "word-break.done", "word-break.fail"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["fits"] as const;

function decode(input: readonly number[]) {
  const words: string[] = [""];
  for (const v of input) {
    if (v === -1) words.push("");
    else words[words.length - 1] += String.fromCharCode(97 + v);
  }
  return words;
}

function trace(input: readonly number[]): Step[] {
  const [text = "", ...words] = decode(input);
  const n = text.length;
  const r = new Recorder([]);
  const ok: (number | null)[] = [1, ...new Array<number | null>(n).fill(null)];
  const snap = (narration: ReturnType<typeof say>, marker: string | null, end: number | null = null, deps: number[] = [], word: number | null = null, span: [number, number] | null = null) => {
    const roles: Record<number, Role> = {};
    if (word !== null) roles[word] = "key";
    r.snap({
      narration,
      marker,
      slots: { layout: "row", label: "letters", cells: [...text].map((ch, id) => ({ id, value: ch.charCodeAt(0) - 97, text: ch })), roles: span ? Object.fromEntries(Array.from({ length: span[1] - span[0] }, (_, k) => [span[0] + k, "compare" as Role])) : {} },
      dp: { label: "breakable", colLabels: Array.from({ length: n + 1 }, (_, k) => String(k)), cells: [[...ok]], hot: end === null ? null : [0, end], deps: deps.map((d) => [0, d]) },
      aux: { label: "dictionary", cells: words.map((w, id) => ({ id, value: w.length, text: w })), roles },
      vars: { end },
    });
  };

  snap(say("word-break.ready", { text, num: words.length }), null);
  for (let end = 1; end <= n; end++) {
    ok[end] = 0;
    snap(say("word-break.end", { end, prefix: text.slice(0, end) }), null, end);
    for (const [k, word] of words.entries()) {
      const start = end - word.length;
      r.comparisons++;
      if (start >= 0 && ok[start] === 1 && text.slice(start, end) === word) {
        ok[end] = 1;
        r.writes++;
        snap(say("word-break.fits", { word, start, end }), "fits", end, [start], k, [start, end]);
        break;
      }
    }
    if (ok[end] === 0) snap(say("word-break.none", { end }), null, end);
  }
  snap(ok[n] === 1 ? say("word-break.done", { text }) : say("word-break.fail", { text }), null);
  r.steps[r.steps.length - 1]!.result = [ok[n] === 1 ? 1 : 0];
  return r.steps;
}

const enc = (text: string, words: string[]) => encodeText([text, ...words]);

export const wordBreak: TracerDef = {
  id: "word-break",
  subject: "dynamic-programming",
  scene: "dp",
  limits: { min_len: 3, max_len: 57, min: -1, max: 25 },
  input: { kind: "codec", codec: textWordsCodec({ maxText: 12, maxWords: 5, maxWord: 8 }) },
  presets: {
    splits: () => enc("sunflowerpot", ["sun", "flow", "flower", "pot", "er"]),
    random: (next) => {
      const pool = ["ab", "ba", "a", "bab", "abb", "b"];
      const words = pool.filter(() => next() < 0.5).slice(0, 4);
      const use = words.length ? words : ["ab"];
      let text = "";
      while (text.length < 6) text += use[Math.floor(next() * use.length)]!;
      if (next() < 0.3) text += "c";
      return enc(text.slice(0, 12), use);
    },
    "dead-end": () => enc("goldfishy", ["gold", "fish", "goldfish", "shy"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "comparisons" },
    { slot: "writes", label: "writes" },
  ],
  legend: ["key", "compare", "sorted"],
  roleLabels: { key: "current", compare: "read", sorted: "filled" },
  vars: ["end"],
  trace,
};
