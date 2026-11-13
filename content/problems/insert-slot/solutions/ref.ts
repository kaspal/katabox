export function insertSlot(slots: number[][], fresh: number[]): number[][] {
  let [s, e] = fresh;
  const out: number[][] = [];
  let i = 0;
  while (i < slots.length && slots[i][1] < s) out.push(slots[i++]);
  while (i < slots.length && slots[i][0] <= e) {
    s = Math.min(s, slots[i][0]);
    e = Math.max(e, slots[i][1]);
    i++;
  }
  out.push([s, e]);
  while (i < slots.length) out.push(slots[i++]);
  return out;
}
