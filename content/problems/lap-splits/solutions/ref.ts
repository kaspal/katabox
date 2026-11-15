import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;

function seconds(lap: string): number {
  const [m, s] = lap.split(":");
  return Number(m) * 60 + Number(s);
}

function clock(total: number): string {
  return `${Math.floor(total / 60)}:${String(total % 60).padStart(2, "0")}`;
}

const t = Number(tokens[pos++]);
const out: string[] = [];
for (let i = 0; i < t; i++) {
  const name = tokens[pos++];
  const k = Number(tokens[pos++]);
  let total = 0;
  let best = Infinity;
  for (let j = 0; j < k; j++) {
    const lap = seconds(tokens[pos++]);
    total += lap;
    if (lap < best) best = lap;
  }
  out.push(`${name} ${clock(total)} ${clock(best)}`);
}
process.stdout.write(out.join("\n") + "\n");
