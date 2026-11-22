const MOD = 1000000007;

export function gridRoutes(floor: string[]): number {
  const rows = floor.length;
  const cols = floor[0].length;
  const memo = new Map<string, number>();
  const routes = (r: number, c: number): number => {
    if (r >= rows || c >= cols || floor[r][c] === "#") return 0;
    if (r === rows - 1 && c === cols - 1) return 1;
    const key = `${r},${c}`;
    const known = memo.get(key);
    if (known !== undefined) return known;
    const total = (routes(r + 1, c) + routes(r, c + 1)) % MOD;
    memo.set(key, total);
    return total;
  };
  return routes(0, 0);
}
