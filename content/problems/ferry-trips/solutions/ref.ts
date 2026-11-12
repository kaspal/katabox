export function minTrips(weights: number[], limit: number): number {
  const w = Int32Array.from(weights).sort();
  let lo = 0;
  let hi = w.length - 1;
  let trips = 0;
  while (lo <= hi) {
    if (w[lo] + w[hi] <= limit) lo++;
    hi--;
    trips++;
  }
  return trips;
}
