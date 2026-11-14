// Suffix array by prefix doubling. Mirrors content/learn/strings/suffix-array/sample.{py,ts}:
// rank every suffix by its first letter; then repeatedly sort by the pair (rank of the
// first k letters, rank of the next k letters), which ranks the first 2k letters, and
// renumber; once all ranks differ, the order is final. Each round doubles k, so there are
// about log n rounds. Output: the starting positions of the suffixes in sorted order.
import { decodeText, encodeText, textCodec } from "../engine/codecs";
import { Recorder } from "../engine/recorder";
import type { NarrationKey, NarrationParams, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"suffix-array">;
const KEYS = ["suffix-array.ready", "suffix-array.sort", "suffix-array.double", "suffix-array.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["sort", "double"] as const;

function trace(input: readonly number[]): Step[] {
  const [text = ""] = decodeText(input);
  const n = text.length;
  const r = new Recorder([]);
  let rank = [...text].map((ch) => ch.charCodeAt(0));
  const order = [...Array(n).keys()];
  let k = 1;
  let shownRank: (number | null)[] = rank.map((v) => v - 97);
  const snap = (narration: ReturnType<typeof say>, marker: string | null) =>
    r.snap({
      narration,
      marker,
      strings: {
        rows: [
          { label: "text", kind: "letters", offset: 0, cells: [...text], roles: {} },
          { label: "rank", kind: "numbers", offset: 0, cells: [...shownRank], roles: {} },
        ],
        lines: order.map((i) => ({ text: text.slice(i), note: `${i}`, role: undefined })),
        linesLabel: "suffixes",
      },
      vars: { k },
    });

  snap(say("suffix-array.ready", { text, n }), null);
  for (;;) {
    const key = (i: number) => [rank[i]!, i + k < n ? rank[i + k]! : -1] as const;
    const cmp = (a: number, b: number) => key(a)[0] - key(b)[0] || key(a)[1] - key(b)[1];
    order.sort(cmp);
    r.comparisons++;
    const next = new Array<number>(n).fill(0);
    for (let j = 1; j < n; j++) next[order[j]!] = next[order[j - 1]!]! + (cmp(order[j]!, order[j - 1]!) !== 0 ? 1 : 0);
    rank = next;
    shownRank = [...rank];
    snap(say("suffix-array.sort", { k, two: 2 * k, distinct: rank[order[n - 1]!]! + 1 }), "sort");
    if (rank[order[n - 1]!] === n - 1) break;
    k *= 2;
    r.moves++;
    snap(say("suffix-array.double", { k }), "double");
  }
  snap(say("suffix-array.done", { n, rounds: r.comparisons }), null);
  r.steps[r.steps.length - 1]!.result = [...order];
  return r.steps;
}

export const suffixArray: TracerDef = {
  id: "suffix-array",
  subject: "strings",
  scene: "string",
  limits: { min_len: 1, max_len: 10, min: 0, max: 25 },
  input: { kind: "codec", codec: textCodec(["text"], { minLen: 1, maxLen: 10 }) },
  presets: {
    "repeated-syllables": () => encodeText(["tatatoma"]),
    random: (next) => encodeText([Array.from({ length: 4 + Math.floor(next() * 7) }, () => "abc"[Math.floor(next() * 3)]!).join("")]),
    "one-letter": () => encodeText(["aaaaa"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "rounds" },
    { slot: "moves", label: "doublings" },
  ],
  legend: [],
  vars: ["k"],
  trace,
};
