function f(n: number): number {
  return hanoi(n, 0, 2, 1);
}

function hanoi(k: number, src: number, dst: number, via: number): number {
  if (k === 0) {
    return 0;
  }
  return hanoi(k - 1, src, via, dst) + 1 + hanoi(k - 1, via, dst, src);
}
