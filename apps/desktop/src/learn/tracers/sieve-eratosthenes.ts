// Sieve of Eratosthenes. Mirrors content/learn/math-geometry/sieve-eratosthenes/sample.{py,ts}:
// write 0..n; for every p with p² ≤ n that is still unmarked (a prime), cross out its
// multiples starting at p² (smaller ones were crossed by smaller primes). What is left
// is prime. Output: how many primes up to n, then the primes. The numbers are drawn ten
// to a row.
import { Recorder } from "../engine/recorder";
import type { NarrationKey, Role, Step, TracerDef } from "../engine/types";

type Key = NarrationKey<"sieve-eratosthenes">;
const KEYS = ["sieve-eratosthenes.ready", "sieve-eratosthenes.prime", "sieve-eratosthenes.cross", "sieve-eratosthenes.again", "sieve-eratosthenes.done"] as const satisfies readonly Key[];
const say = (key: (typeof KEYS)[number], params?: Record<string, number>) => ({ key, params });

export const MARKERS = ["prime", "cross"] as const;

function trace(input: readonly number[]): Step[] {
  const n = input[0]!;
  const r = new Recorder([]);
  const isPrime = new Array<boolean>(n + 1).fill(true);
  isPrime[0] = false;
  isPrime[1] = false;
  let p = 2;
  let finished = false;
  const rows = Math.ceil((n + 1) / 10);
  const snap = (narration: ReturnType<typeof say>, marker: string | null, hot: number | null = null, role: Role = "key") => {
    const roles: Record<number, Role> = {};
    for (let k = 0; k <= n; k++) {
      // Crossed out; or known prime (below p², or the sieve is done); or still open.
      if (!isPrime[k]) roles[k] = "out";
      else if (finished || k < p * p) roles[k] = "sorted";
    }
    if (hot !== null) roles[hot] = role;
    r.snap({
      narration,
      marker,
      board: { rows, cols: 10, cells: Array.from({ length: rows * 10 }, (_, k) => (k <= n ? String(k) : "")), roles },
      vars: { p, primes: isPrime.filter(Boolean).length },
    });
  };

  snap(say("sieve-eratosthenes.ready", { n }), null);
  for (p = 2; p * p <= n; p++) {
    if (isPrime[p]) {
      snap(say("sieve-eratosthenes.prime", { p, start: p * p }), "prime", p, "pivot");
      for (let multiple = p * p; multiple <= n; multiple += p) {
        r.comparisons++;
        const fresh = isPrime[multiple]!;
        isPrime[multiple] = false;
        if (fresh) r.writes++;
        snap(fresh ? say("sieve-eratosthenes.cross", { m: multiple, p }) : say("sieve-eratosthenes.again", { m: multiple, p }), "cross", multiple, "compare");
      }
    }
  }
  finished = true;
  const primes = isPrime.flatMap((b, k) => (b ? [k] : []));
  snap(say("sieve-eratosthenes.done", { n, count: primes.length, crossed: r.comparisons }), null);
  r.steps[r.steps.length - 1]!.result = [primes.length, ...primes];
  return r.steps;
}

export const sieveEratosthenes: TracerDef = {
  id: "sieve-eratosthenes",
  subject: "math-geometry",
  scene: "board",
  limits: { min_len: 1, max_len: 1, min: 2, max: 120 },
  input: { kind: "count" },
  markers: MARKERS,
  narrationKeys: KEYS,
  counters: [
    { slot: "comparisons", label: "crossings" },
    { slot: "writes", label: "newly_crossed" },
  ],
  legend: ["pivot", "compare", "sorted", "out"],
  roleLabels: { pivot: "current_prime", compare: "crossed_now", sorted: "prime", out: "crossed" },
  vars: ["p", "primes"],
  trace,
};
