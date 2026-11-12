export function mostSnacks(prices: number[], budget: number): number {
  const sorted = Int32Array.from(prices).sort();
  let spent = 0;
  let count = 0;
  for (const p of sorted) {
    if (spent + p > budget) break;
    spent += p;
    count++;
  }
  return count;
}
