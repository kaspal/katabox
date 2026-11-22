function makeAcc(start: number): (x: number) => number {
  let total = start;
  return (x: number) => {
    total += x;
    return total;
  };
}

const acc = makeAcc(10);
acc(5);
console.log(acc(1));
const other = makeAcc(0);
console.log(other(2));
console.log(acc(0));
