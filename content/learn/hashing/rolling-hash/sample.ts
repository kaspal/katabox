const BASE = 26;
const MOD = 101;

function windowHashes(text: string, length: number): number[] {
  let power = 1;
  for (let k = 0; k < length - 1; k++) power = (power * BASE) % MOD;
  let h = 0;
  for (const c of text.slice(0, length)) {
    h = (h * BASE + c.charCodeAt(0) - 97) % MOD; // @step add
  }
  const out = [h];
  for (let i = length; i < text.length; i++) {
    h = (((h - (text.charCodeAt(i - length) - 97) * power) % MOD) + MOD) % MOD; // @step remove
    h = (h * BASE + text.charCodeAt(i) - 97) % MOD; // @step shift-in
    out.push(h);
  }
  return out;
}
