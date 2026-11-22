function wrap(n: number): void {
  if (n === 0) {
    console.log("base");
    return;
  }
  console.log("in", n);
  wrap(n - 1);
  console.log("out", n);
}

wrap(2);
