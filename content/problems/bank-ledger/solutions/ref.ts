import { readFileSync } from "node:fs";

const lines = readFileSync(0, "utf8").split("\n");
const q = Number(lines[0]);
const balance = new Map<string, number>();
const holders = new Map<number, number>(); // balance → how many accounts hold it (for COUNT)
// One undo log for all open transactions: [name, balance before or null]. BEGIN remembers
// where the log stood; ROLLBACK undoes back to there; COMMIT only forgets the mark, so its
// changes now belong to the enclosing transaction.
const log: [string, number | null][] = [];
const marks: number[] = [];

function put(name: string, value: number | null): void {
  const old = balance.get(name);
  if (old !== undefined) holders.set(old, holders.get(old)! - 1);
  if (value === null) balance.delete(name);
  else {
    balance.set(name, value);
    holders.set(value, (holders.get(value) ?? 0) + 1);
  }
}

function change(name: string, value: number): void {
  if (marks.length > 0) log.push([name, balance.get(name) ?? null]);
  put(name, value);
}

const out: string[] = [];
for (let i = 1; i <= q; i++) {
  const parts = lines[i].trim().split(" ");
  switch (parts[0]) {
    case "SET":
      change(parts[1], Number(parts[2]));
      break;
    case "ADD":
      change(parts[1], (balance.get(parts[1]) ?? 0) + Number(parts[2]));
      break;
    case "GET":
      out.push(String(balance.get(parts[1]) ?? "NONE"));
      break;
    case "COUNT":
      out.push(String(holders.get(Number(parts[1])) ?? 0));
      break;
    case "BEGIN":
      marks.push(log.length);
      break;
    default: // ROLLBACK or COMMIT
      if (marks.length === 0) out.push("NO TRANSACTION");
      else if (parts[0] === "ROLLBACK") {
        const mark = marks.pop()!;
        while (log.length > mark) {
          const [name, old] = log.pop()!;
          put(name, old);
        }
      } else {
        marks.pop();
        if (marks.length === 0) log.length = 0; // nothing can be rolled back any more
      }
  }
}
process.stdout.write(out.length > 0 ? out.join("\n") + "\n" : "");
