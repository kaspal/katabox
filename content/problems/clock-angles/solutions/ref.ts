import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const q = Number(tokens[0]);
const out: string[] = [];
for (let i = 1; i <= q; i++) {
  const t = tokens[i];
  const h = Number(t.slice(0, 2));
  const m = Number(t.slice(3));
  // In half-degrees, so everything stays whole: the hour hand is at 60·h + m,
  // the minute hand at 12·m.
  let gap = Math.abs(60 * (h % 12) - 11 * m);
  gap = Math.min(gap, 720 - gap);
  out.push((gap / 2).toFixed(1));
}
process.stdout.write(out.join("\n") + "\n");
