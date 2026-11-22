export function spanSums(rain: number[], queries: number[][]): number[] {
  const prefix = [0];
  for (const x of rain) prefix.push(prefix[prefix.length - 1] + x);
  return queries.map(([l, r]) => prefix[r + 1] - prefix[l]);
}
