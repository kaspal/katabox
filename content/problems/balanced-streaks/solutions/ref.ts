export function balancedStretches(results: string): number {
  // Scores range from -n to n: an array indexed by score + n.
  const n = results.length;
  const seen = new Int32Array(2 * n + 1);
  seen[n] = 1;
  let score = 0;
  let count = 0;
  for (let i = 0; i < n; i++) {
    const c = results[i];
    if (c === "W") score++;
    else if (c === "L") score--;
    count += seen[score + n];
    seen[score + n]++;
  }
  return count;
}
