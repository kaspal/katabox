import { readFileSync } from "node:fs";

const lines = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(lines[0]);
const total = new Map<string, number>();
for (let i = 1; i <= n; i++) {
  for (const entry of lines[i].split(";")) {
    const [name, qty] = entry.split(":");
    total.set(name, (total.get(name) ?? 0) + Number(qty));
  }
}
// Plain comparison, not localeCompare: lowercase ASCII names in code-point order.
const names = [...total.keys()].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
process.stdout.write(names.map((name) => `${name} ${total.get(name)}`).join("\n") + "\n");
