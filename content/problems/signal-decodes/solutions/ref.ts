import { readFileSync } from "node:fs";

const MOD = 1_000_003;

function ways(msg: string): number {
  // msg[0] is the '#'; the digits start at index 1.
  // before = ways of the prefix one digit shorter, last = ways of the current prefix.
  let before = 1;
  let last = msg.charCodeAt(1) === 48 ? 0 : 1;
  for (let i = 2; i < msg.length; i++) {
    const c = msg.charCodeAt(i);
    const p = msg.charCodeAt(i - 1);
    let cur = c !== 48 ? last : 0;
    if (p === 49 || (p === 50 && c <= 54)) cur += before; // "1x" or "20".."26"
    before = last;
    last = cur % MOD;
  }
  return last;
}

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const t = Number(tokens[0]);
const out: string[] = [];
for (let j = 1; j <= t; j++) out.push(String(ways(tokens[j])));
process.stdout.write(out.join("\n") + "\n");
