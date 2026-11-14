// KataBox Learn sample check (ADR 0029): kb-pack wraps sample.ts in this; never shown
// in the app. The import comes first: `node --check` only strips types from a .ts file
// that starts like an ES module.
import { readFileSync as kbReadFileSync } from "node:fs";

{{sample}}

// Read the values, call the sample on the array `a`, print it.
const a: number[] = kbReadFileSync(0, "utf8")
  .split(/\s+/)
  .filter((token) => token !== "")
  .map(Number);
{{call}};
console.log(a.join(" "));
