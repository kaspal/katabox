import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const minutes = (t: string): number => Number(t.slice(0, 2)) * 60 + Number(t.slice(3));
const pad = (x: number): string => String(x).padStart(2, "0");
const clock = (m: number): string => `${pad(Math.floor(m / 60))}:${pad(m % 60)}`;

const t = Number(tokens[pos++]);
const out: string[] = [];
for (let d = 0; d < t; d++) {
  const open = minutes(tokens[pos++]);
  const close = minutes(tokens[pos++]);
  const k = Number(tokens[pos++]);
  const slots: [number, number][] = [];
  for (let i = 0; i < k; i++) slots.push([minutes(tokens[pos++]), minutes(tokens[pos++])]);
  slots.sort((a, b) => a[0] - b[0]);
  // Sweep the slots by start time; `freeFrom` is where the free time begins. A gap is
  // only replaced by a strictly longer one, so the earliest longest gap wins.
  let bestLen = 0;
  let bestStart = 0;
  let freeFrom = open;
  for (const [s, e] of slots) {
    if (s - freeFrom > bestLen) {
      bestLen = s - freeFrom;
      bestStart = freeFrom;
    }
    freeFrom = Math.max(freeFrom, e);
  }
  if (close - freeFrom > bestLen) {
    bestLen = close - freeFrom;
    bestStart = freeFrom;
  }
  out.push(bestLen > 0 ? `${clock(bestStart)}-${clock(bestStart + bestLen)}` : "BOOKED");
}
process.stdout.write(out.join("\n") + "\n");
