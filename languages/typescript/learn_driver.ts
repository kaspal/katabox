// KataBox Learn sample check (ADR 0029): kb-pack wraps sample.ts in this; never shown
// in the app. The import comes first (once needed by the old `node --check` syntax
// check; ADR 0029).
import { readFileSync as kbReadFileSync } from "node:fs";

{{sample}}

// Stored operations as tuples: [name, arg, ...], e.g. ["push", 3] or ["pop"].
function kbOps(values: number[], names: string[], width: number): (string | number)[][] {
  const out: (string | number)[][] = [];
  for (let k = 0; k < values.length; k += width + 1) {
    const name = names[values[k] - 1];
    out.push([name, ...values.slice(k + 1, k + 1 + width)]);
  }
  return out;
}

// Stored words (letter codes 0-25, words separated by -1) as strings.
function kbText(values: number[]): string[] {
  const words = [""];
  for (const v of values) {
    if (v === -1) words.push("");
    else words[words.length - 1] += String.fromCharCode(97 + v);
  }
  return words;
}

// A stored graph [n, directed, u, v, w, ...]: its edges as [u, v, w], in order.
function kbEdges(values: number[]): number[][] {
  const out: number[][] = [];
  for (let k = 2; k < values.length; k += 3) out.push(values.slice(k, k + 3));
  return out;
}

// A stored graph as [n, adjacency lists of [v, w]]; undirected edges go both ways.
function kbGraph(values: number[]): [number, number[][][]] {
  const n = values[0];
  const adj: number[][][] = Array.from({ length: n }, () => []);
  for (const [u, v, w] of kbEdges(values)) {
    adj[u].push([v, w]);
    if (!values[1]) adj[v].push([u, w]);
  }
  return [n, adj];
}

// Stored groups (pairs, edges) as arrays of `size` values.
function kbChunks(values: number[], size: number): number[][] {
  const out: number[][] = [];
  for (let k = 0; k < values.length; k += size) out.push(values.slice(k, k + size));
  return out;
}

// Read the values, call the sample on the array `a` (the call may also set `a` to the
// sample's result), print it.
let a: number[] = kbReadFileSync(0, "utf8")
  .split(/\s+/)
  .filter((token) => token !== "")
  .map(Number);
{{call}};
console.log(a.join(" "));
