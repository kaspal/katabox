const a = [1, 2, 3];
const b = [...a];
const c = Array.from(a);
b[0] = 10;
c.push(99);
console.log(a.join(" "));
console.log(b.join(" "));
console.log(c.length);
