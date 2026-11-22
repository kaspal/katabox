const fns: (() => number)[] = [];
for (let i = 0; i < 3; i++) {
  fns.push(() => i);
}

console.log(fns.map((f) => f()).join(" "));
