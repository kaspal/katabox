export function tripTotals(miles: number[], trips: number[][]): number[] {
  const prefix = [0];
  for (const m of miles) prefix.push(prefix[prefix.length - 1] + m);
  return trips.map(([a, b]) => prefix[b + 1] - prefix[a]);
}
