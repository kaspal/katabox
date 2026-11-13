export function temperatureSwing(readings: number[]): number {
  let warmest = readings[0];
  let coldest = readings[0];
  for (const r of readings) {
    if (r > warmest) warmest = r;
    if (r < coldest) coldest = r;
  }
  return warmest - coldest;
}
