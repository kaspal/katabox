import { readFileSync } from "node:fs";

const input = readFileSync(0, "utf8");
const newline = input.indexOf("\n");
const k = Number(input.slice(0, newline));
const counts = new Map<string, number>();
for (const word of input.slice(newline + 1).match(/[a-z]+/g) ?? []) {
  counts.set(word, (counts.get(word) ?? 0) + 1);
}
const ranked = [...counts.entries()].sort((a, b) => {
  if (a[1] !== b[1]) return b[1] - a[1];
  return a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0;
});
const out: string[] = [];
for (const [word, count] of ranked.slice(0, k)) out.push(`${word} ${count}`);
process.stdout.write(out.join("\n") + "\n");
