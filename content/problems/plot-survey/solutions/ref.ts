import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
const xs: number[] = [];
const ys: number[] = [];
for (let i = 0; i < n; i++) {
  xs.push(Number(tokens[1 + 2 * i]));
  ys.push(Number(tokens[2 + 2 * i]));
}
const gcd = (a: number, b: number): number => (b === 0 ? a : gcd(b, a % b));

// Each shoelace term is at most 2·10¹² and the sum at most about 8·10¹²: exact in a double.
let twice = 0; // twice the area: the shoelace formula, in whole numbers
let boundary = 0; // lattice points on the edges: an edge (dx, dy) holds gcd(|dx|, |dy|)
for (let i = 0; i < n; i++) {
  const j = (i + 1) % n;
  twice += xs[i] * ys[j] - xs[j] * ys[i];
  boundary += gcd(Math.abs(xs[j] - xs[i]), Math.abs(ys[j] - ys[i]));
}
twice = Math.abs(twice);
// Pick's theorem: area = inside + boundary / 2 - 1, so inside = (2·area - boundary + 2) / 2.
const inside = (twice - boundary + 2) / 2;
process.stdout.write(`${Math.floor(twice / 2)}.${twice % 2 === 1 ? 5 : 0} ${inside}\n`);
