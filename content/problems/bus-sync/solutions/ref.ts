import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

// The remainder in [0, m), also for negative v.
const mod = (v: number, m: number): number => ((v % m) + m) % m;

function gcd(a: number, b: number): number {
  while (b !== 0) {
    const r = a % b;
    a = b;
    b = r;
  }
  return a;
}

// a⁻¹ modulo m (gcd(a, m) = 1) by the extended Euclidean algorithm; 0 when m = 1.
function inverse(a: number, m: number): number {
  let oldR = a;
  let r = m;
  let oldS = 1;
  let s = 0;
  while (r !== 0) {
    const q = Math.floor(oldR / r);
    const nr = oldR - q * r;
    oldR = r;
    r = nr;
    const ns = oldS - q * s;
    oldS = s;
    s = ns;
  }
  return mod(oldS, m);
}

const t = next();
const out: string[] = [];
for (let c = 0; c < t; c++) {
  const n = next();
  let x = 0; // the lines so far all leave at minutes x + big·k
  let big = 1;
  let ok = true;
  for (let i = 0; i < n; i++) {
    const r = next();
    const m = next();
    if (!ok) continue;
    const g = gcd(big, m);
    if ((r - x) % g !== 0) {
      ok = false;
      continue;
    }
    const step = m / g;
    // Both factors are below step ≤ 10⁶ (product < 10¹²), and big·k < big·step = lcm ≤ 9·10¹⁵
    // < 2⁵³: every value is an exact double.
    const k = (mod((r - x) / g, step) * inverse(mod(big / g, step), step)) % step;
    x += big * k;
    big *= step;
  }
  out.push(ok ? String(x) : "-1");
}
process.stdout.write(out.join("\n") + "\n");
