const M = 16;
const HASHES = [
  [1, 0],
  [7, 3],
  [11, 5],
];

function positions(key: number): number[] {
  return HASHES.map(([a, b]) => (key * a + b) % M);
}

function run(ops: (string | number)[][]): number[] {
  const bits = new Array<number>(M).fill(0);
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "add") {
      for (const p of positions(op[1] as number)) {
        bits[p] = 1; // @step set
      }
    } else {
      let maybe = 1;
      for (const p of positions(op[1] as number)) {
        if (bits[p] === 0) { // @step check
          maybe = 0;
          break;
        }
      }
      out.push(maybe); // @step answer
    }
  }
  return out;
}
