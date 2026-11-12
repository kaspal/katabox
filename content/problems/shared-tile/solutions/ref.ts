export function largestTile(width: number, height: number): number {
  let a = width;
  let b = height;
  while (b !== 0) {
    const r = a % b;
    a = b;
    b = r;
  }
  return a;
}
