let a = [1];
const b = a;
a.push(2);
a = [...a, 3];
a.push(4);
console.log(a.join(" "));
console.log(b.join(" "));
