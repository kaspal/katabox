function makeCounter(): () => number {
  let count = 0;
  return () => {
    count += 1;
    return count;
  };
}

const a = makeCounter();
const b = makeCounter();
console.log(a());
console.log(a());
console.log(b());
