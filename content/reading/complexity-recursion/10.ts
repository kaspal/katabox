function f(n: number): number {
  return subsets(0, 0, n);
}

function subsets(i: number, total: number, n: number): number {
  if (i === n) {
    return total % 3 === 0 ? 1 : 0;
  }
  return subsets(i + 1, total + i, n) + subsets(i + 1, total, n);
}
