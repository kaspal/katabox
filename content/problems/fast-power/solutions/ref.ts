const P = 1_000_003;

export function powerMod(base: number, exponent: number): number {
  let result = 1;
  let square = base % P;
  let e = exponent;
  while (e > 0) {
    if (e % 2 === 1) result = (result * square) % P;
    square = (square * square) % P;
    e = Math.floor(e / 2);
  }
  return result;
}
