// Aho-Corasick: many patterns in one pass. Mirrors content/learn/strings/aho-corasick/sample.{py,ts}:
// put the patterns in a trie; give every node a failure link to the longest proper suffix
// of its word that is also in the trie (breadth-first, so shorter nodes are done first);
// then read the text once, following failure links on a missing edge, and count every
// pattern that ends at the current node (its own and those reached through failure
// links). Output: how many times each pattern occurs, in input order. The count so far
// sits under the node where each pattern ends.
import { textWordsCodec } from "../engine/shapes";
import { encodeText } from "../engine/codecs";
import { Recorder } from "../engine/recorder";
import type { ForestNode, NarrationKey, NarrationParams, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"aho-corasick">;
const KEYS = ["aho-corasick.ready", "aho-corasick.built", "aho-corasick.link", "aho-corasick.read", "aho-corasick.follow", "aho-corasick.hit", "aho-corasick.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: NarrationParams) => ({ key, params: params as NarrationParams<number> });

export const MARKERS = ["link", "follow", "hit"] as const;

function decode(input: readonly number[]) {
  const words: string[] = [""];
  for (const v of input) {
    if (v === -1) words.push("");
    else words[words.length - 1] += String.fromCharCode(97 + v);
  }
  return words;
}

function trace(input: readonly number[]): Step[] {
  const [text = "", ...patterns] = decode(input);
  const r = new Recorder([]);
  const go: Map<string, number>[] = [new Map()];
  const parent = [-1];
  const letter = [""];
  const fail = [0];
  const own: number[][] = [[]];
  const out: number[][] = [[]];
  patterns.forEach((word, p) => {
    let node = 0;
    for (const ch of word) {
      if (!go[node]!.has(ch)) {
        go.push(new Map());
        parent.push(node);
        letter.push(ch);
        fail.push(0);
        own.push([]);
        out.push([]);
        go[node]!.set(ch, go.length - 1);
      }
      node = go[node]!.get(ch)!;
    }
    own[node]!.push(p);
    out[node]!.push(p);
  });
  const linked = new Set<number>();
  const counts = new Array<number>(patterns.length).fill(0);
  let at = -1;
  let node = 0;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}) => {
    const forest: ForestNode[] = go.map((_, id) => ({
      id,
      parent: id === 0 ? null : parent[id]!,
      text: id === 0 ? "·" : letter[id]!,
      role: roles[id] ?? (id === node && at >= 0 ? "key" : undefined),
      mark: own[id]!.length > 0,
      note: own[id]!.length ? own[id]!.map((p) => `${patterns[p]}·${counts[p]}`).join(" ") : undefined,
    }));
    r.snap({
      narration,
      marker,
      forest,
      links: [...linked].filter((id) => fail[id]! !== 0).map((id) => ({ from: id, to: fail[id]! })),
      slots: { layout: "row", label: "text", cells: [...text].map((ch, id) => ({ id, value: id, text: ch })), roles: at >= 0 ? { [at]: "key" } : {} },
      vars: { node, at: at >= 0 ? at : null },
    });
  };

  snap(say("aho-corasick.ready", { num: patterns.length }), null);
  snap(say("aho-corasick.built", { nodes: go.length }), null);
  const queue = [...go[0]!.values()];
  for (const id of queue) linked.add(id);
  while (queue.length) {
    const cur = queue.shift()!;
    for (const [ch, child] of go[cur]!) {
      let f = fail[cur]!;
      while (f && !go[f]!.has(ch)) f = fail[f]!;
      fail[child] = go[f]!.get(ch) ?? 0;
      out[child] = [...out[child]!, ...out[fail[child]!]!];
      linked.add(child);
      r.writes++;
      snap(say("aho-corasick.link", { letter: ch, to: fail[child] === 0 ? "·" : letter[fail[child]!]! }), "link", { [child]: "key", [fail[child]!]: "compare" });
      queue.push(child);
    }
  }
  for (let i = 0; i < text.length; i++) {
    at = i;
    const ch = text[i]!;
    while (node && !go[node]!.has(ch)) {
      const from = node;
      node = fail[node]!;
      r.moves++;
      snap(say("aho-corasick.follow", { letter: ch }), "follow", { [from]: "out", [node]: "compare" });
    }
    node = go[node]!.get(ch) ?? 0;
    r.comparisons++;
    snap(say("aho-corasick.read", { letter: ch }), null);
    for (const p of out[node]!) {
      counts[p]!++;
      snap(say("aho-corasick.hit", { word: patterns[p]!, end: i }), "hit", { [node]: "sorted" });
    }
  }
  at = -1;
  snap(say("aho-corasick.done", { reads: r.comparisons, follows: r.moves }), null);
  r.steps[r.steps.length - 1]!.result = counts;
  return r.steps;
}

export const ahoCorasick: TracerDef = {
  id: "aho-corasick",
  subject: "strings",
  scene: "tree",
  limits: { min_len: 3, max_len: 41, min: -1, max: 25 },
  input: { kind: "codec", codec: textWordsCodec({ maxText: 14, maxWords: 4, maxWord: 5 }) },
  presets: {
    "overlaps": () => encodeText(["abcabcab", "ab", "bc", "abc", "cab"]),
    random: (next) => {
      const word = (n: number) => Array.from({ length: n }, () => "ab"[Math.floor(next() * 2)]!).join("");
      const pats = [...new Set([word(2), word(2), word(3)])];
      return encodeText([word(9 + Math.floor(next() * 5)), ...pats]);
    },
    "no-overlap": () => encodeText(["catdogcow", "cat", "dog", "pig"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "letters_read" },
    { slot: "moves", label: "fail_links_followed" },
    { slot: "writes", label: "links" },
  ],
  legend: ["key", "compare", "sorted", "out"],
  roleLabels: { key: "current", compare: "failure_target", sorted: "match", out: "left_behind" },
  vars: ["node", "at"],
  trace,
};
