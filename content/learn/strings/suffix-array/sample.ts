function suffixArray(text: string): number[] {
  const n = text.length;
  let rank = [...text].map((ch) => ch.charCodeAt(0));
  const order = [...Array(n).keys()];
  let k = 1;
  for (;;) {
    const key = (i: number) => [rank[i], i + k < n ? rank[i + k] : -1];
    const cmp = (a: number, b: number) =>
      key(a)[0] - key(b)[0] || key(a)[1] - key(b)[1];
    order.sort(cmp); // @step sort
    const next = new Array<number>(n).fill(0);
    for (let j = 1; j < n; j++) {
      const changed = cmp(order[j], order[j - 1]) !== 0 ? 1 : 0;
      next[order[j]] = next[order[j - 1]] + changed;
    }
    rank = next;
    if (rank[order[n - 1]] === n - 1) return order;
    k *= 2; // @step double
  }
}
