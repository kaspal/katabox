// Trie (prefix tree). Mirrors content/learn/trees/trie/sample.{py,ts}: each edge is a letter;
// inserting a word follows existing edges and adds the missing ones, counting at every node
// how many words pass through it and marking where a word ends. Counting the words with a
// prefix then walks the prefix and reads one count: O(prefix length), whatever the number
// of words.
import { decodeText, encodeText, fail } from "../engine/codecs";
import { int } from "../engine/inputs";
import { Recorder } from "../engine/recorder";
import type { ForestNode, NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"trie">;
const KEYS = [
  "trie.ready",
  "trie.new_node",
  "trie.follow",
  "trie.mark_end",
  "trie.walk",
  "trie.missing",
  "trie.count_word",
  "trie.count_only",
] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number | string>) => ({ key, params });

export const MARKERS = ["new-node", "follow", "mark-end", "missing", "walk", "count"] as const;

type TNode = { id: number; parent: number | null; letter: string; children: Map<string, number>; words: number; end: boolean };

function trace(input: readonly number[]): Step[] {
  const all = decodeText(input);
  const words = all.slice(0, -1);
  const prefix = all[all.length - 1]!;
  const r = new Recorder([]);
  const nodes = new Map<number, TNode>();
  nodes.set(0, { id: 0, parent: null, letter: "", children: new Map(), words: 0, end: false });
  let nextId = 1;
  const snap = (narration: ReturnType<typeof say>, marker: string | null, roles: Record<number, Role> = {}, path = "") => {
    const forest: ForestNode[] = [];
    const walk = (id: number) => {
      const n = nodes.get(id)!;
      forest.push({ id, parent: n.parent, text: id === 0 ? "·" : n.letter, role: roles[id], mark: n.end, ...(id === 0 ? {} : { note: String(n.words) }) });
      for (const c of [...n.children.keys()].sort()) walk(n.children.get(c)!);
    };
    walk(0);
    r.snap({ narration, marker, forest, vars: { path: path || null, nodes: nodes.size } });
  };

  snap(say("trie.ready", { n: words.length, prefix }), null);
  for (const word of words) {
    let node = 0;
    let path = "";
    for (const c of word) {
      path += c;
      const cur = nodes.get(node)!;
      if (!cur.children.has(c)) {
        const id = nextId++;
        nodes.set(id, { id, parent: node, letter: c, children: new Map(), words: 0, end: false });
        cur.children.set(c, id);
        r.writes++;
        snap(say("trie.new_node", { letter: c, path }), "new-node", { [id]: "swap" }, path);
      }
      node = cur.children.get(c)!;
      nodes.get(node)!.words++;
      r.comparisons++;
      snap(say("trie.follow", { letter: c, count: nodes.get(node)!.words }), "follow", { [node]: "compare" }, path);
    }
    nodes.get(node)!.end = true;
    snap(say("trie.mark_end", { word }), "mark-end", { [node]: "sorted" }, word);
  }
  let node = 0;
  let path = "";
  for (const c of prefix) {
    const cur = nodes.get(node)!;
    if (!cur.children.has(c)) {
      snap(say("trie.missing", { prefix, letter: c }), "missing", { [node]: "out" }, path);
      r.steps[r.steps.length - 1]!.result = [0, 0];
      return r.steps;
    }
    node = cur.children.get(c)!;
    path += c;
    r.comparisons++;
    snap(say("trie.walk", { letter: c, path }), "walk", { [node]: "key" }, path);
  }
  const n = nodes.get(node)!;
  const count = n.words;
  snap(say(n.end ? "trie.count_word" : "trie.count_only", { prefix, count }), "count", { [node]: "sorted" }, path);
  r.steps[r.steps.length - 1]!.result = [count, n.end ? 1 : 0];
  return r.steps;
}

const WORDS = ["car", "cart", "care", "cat", "dog", "dot", "do", "card", "cot", "tea", "ten", "to"];

export const trie: TracerDef = {
  id: "trie",
  subject: "trees",
  scene: "tree",
  limits: { min_len: 3, max_len: 60, min: -1, max: 25 },
  input: {
    kind: "codec",
    codec: {
      fields: ["words", "pattern"],
      format: (input) => {
        const all = decodeText(input);
        return [all.slice(0, -1).join(" "), all[all.length - 1] ?? ""];
      },
      parse: (texts) => {
        const words = (texts[0] ?? "").trim().toLowerCase().split(/[\s,]+/).filter((w) => w !== "");
        const prefix = (texts[1] ?? "").trim().toLowerCase();
        if (words.length < 1 || words.length > 6) return fail("word_count", { min: 1, max: 6 });
        for (const w of [...words, prefix]) {
          if (!/^[a-z]+$/.test(w)) return fail("letters_only", { field: w === prefix ? "pattern" : "words" });
          if (w.length > 6) return fail("word_length", { field: w === prefix ? "pattern" : "words", min: 1, max: 6 });
        }
        return { values: encodeText([...words, prefix]) };
      },
      check: (input) => {
        const all = decodeText(input);
        if (all.length < 2 || all.length > 7 || all.some((w) => w.length < 1 || w.length > 6)) return { code: "custom", key: "size" };
        return null;
      },
    },
  },
  presets: {
    random: (next) => {
      const k = int(next, 3, 5);
      const pool = [...WORDS];
      const words: string[] = [];
      for (let j = 0; j < k; j++) words.push(pool.splice(int(next, 0, pool.length - 1), 1)[0]!);
      const pick = words[int(next, 0, words.length - 1)]!;
      return encodeText([...words, pick.slice(0, int(next, 1, pick.length))]);
    },
    "shared-prefixes": () => encodeText(["car", "cart", "care", "cat", "dog", "ca"]),
    "missing-prefix": () => encodeText(["tea", "ten", "to", "inn", "tx"]),
  },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "writes", label: "nodes_added" },
    { slot: "comparisons", label: "edges_followed" },
  ],
  legend: ["swap", "compare", "key", "sorted", "out"],
  roleLabels: { swap: "new_node", compare: "following", key: "walking", sorted: "word_end", out: "no_edge" },
  vars: ["path", "nodes"],
  trace,
};
