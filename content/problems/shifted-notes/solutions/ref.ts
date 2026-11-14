import { readFileSync } from "node:fs";

const data = readFileSync(0);
const newline = data.indexOf(10);
const s = Number(data.toString("latin1", 0, newline));
const text = data.subarray(newline + 1);
for (let i = 0; i < text.length; i++) {
  const c = text[i];
  if (c >= 97 && c <= 122) {
    text[i] = ((c - 97 - s + 26) % 26) + 97; // a-z
  } else if (c >= 65 && c <= 90) {
    text[i] = ((c - 65 - s + 26) % 26) + 65; // A-Z
  }
}
process.stdout.write(text);
