export function windowSwings(readings: number[], k: number): number[] {
  // Each queue is an array with a head index (indices only ever leave from the front).
  const maxq = new Int32Array(readings.length);
  const minq = new Int32Array(readings.length);
  let maxHead = 0;
  let maxTail = 0;
  let minHead = 0;
  let minTail = 0;
  const out: number[] = [];
  for (let i = 0; i < readings.length; i++) {
    const r = readings[i];
    while (maxTail > maxHead && readings[maxq[maxTail - 1]] <= r) maxTail--;
    maxq[maxTail++] = i;
    while (minTail > minHead && readings[minq[minTail - 1]] >= r) minTail--;
    minq[minTail++] = i;
    if (maxq[maxHead] <= i - k) maxHead++;
    if (minq[minHead] <= i - k) minHead++;
    if (i >= k - 1) out.push(readings[maxq[maxHead]] - readings[minq[minHead]]);
  }
  return out;
}
