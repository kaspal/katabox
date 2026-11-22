const out: number[] = [];

function tree(n: number): void {
  if (n === 0) {
    return;
  }
  tree(n - 1);
  out.push(n);
  tree(n - 1);
}

tree(3);
console.log(out.join(" "));
