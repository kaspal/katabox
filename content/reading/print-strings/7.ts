let s = "";
for (let i = 1; i < 6; i++) {
  if (i % 2 === 1) {
    s = s + String(i);
  } else {
    s = String(i) + s;
  }
}
console.log(s);
