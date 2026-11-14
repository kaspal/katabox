// The six growth classes of the Big-O topic: each one's step count for n, exactly as the
// sample's loops count them (shared by the tracer and the chart).
const log = (n: number) => Math.floor(Math.log2(n)) + 1; // passes of a doubling loop

export const GROWTH_CLASSES = [
  { label: "O(1)", f: (_n: number) => 1 },
  { label: "O(log n)", f: (n: number) => log(n) },
  { label: "O(n)", f: (n: number) => n },
  { label: "O(n log n)", f: (n: number) => n * log(n) },
  { label: "O(n²)", f: (n: number) => n * n },
  { label: "O(2ⁿ)", f: (n: number) => 2 ** n },
] as const;

export const GROWTH_MAX_N = 16;
