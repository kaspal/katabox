import { readFileSync } from "node:fs";

const s = readFileSync(0, "utf8").trim();
// The longest palindrome at the start of s is the longest prefix of s that is also a
// suffix of reversed(s). The prefix function of s + "#" + reversed(s) gives exactly that
// at its last position ("#" stops a match from running across the middle).
const reversed = s.split("").reverse().join("");
const t = s + "#" + reversed;
const pi = new Int32Array(t.length);
for (let i = 1; i < t.length; i++) {
  let k = pi[i - 1];
  while (k > 0 && t.charCodeAt(i) !== t.charCodeAt(k)) k = pi[k - 1];
  if (t.charCodeAt(i) === t.charCodeAt(k)) k++;
  pi[i] = k;
}
const keep = pi[t.length - 1];
process.stdout.write(reversed.slice(0, s.length - keep) + s + "\n");
