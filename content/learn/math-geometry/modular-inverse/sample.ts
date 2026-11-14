function modInverse(a: number, m: number): number[] {
  let [oldR, r] = [a % m, m];
  let [oldX, x] = [1, 0];
  while (r !== 0) {
    const q = Math.floor(oldR / r);
    [oldR, r] = [r, oldR - q * r]; // @step divide
    [oldX, x] = [x, oldX - q * x];
  }
  if (oldR !== 1) return [-1];
  return [((oldX % m) + m) % m];
}
