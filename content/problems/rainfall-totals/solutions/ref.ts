export function rangeTotals(rain: number[], queries: number[][]): number[] {
  const prefix = new Array<number>(rain.length + 1).fill(0);
  for (let d = 0; d < rain.length; d++) prefix[d + 1] = prefix[d] + rain[d];
  return queries.map(([first, last]) => prefix[last + 1] - prefix[first]);
}
