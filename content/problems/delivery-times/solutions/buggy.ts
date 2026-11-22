/** A binary min-heap of [distance, node] pairs. */
class PairHeap {
  private items: [number, number][] = [];
  get size(): number {
    return this.items.length;
  }
  push(x: [number, number]): void {
    const a = this.items;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p][0] <= a[i][0]) break;
      [a[i], a[p]] = [a[p], a[i]];
      i = p;
    }
  }
  pop(): [number, number] {
    const a = this.items;
    const top = a[0];
    const last = a.pop() as [number, number];
    if (a.length > 0) {
      a[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < a.length && a[l][0] < a[m][0]) m = l;
        if (r < a.length && a[r][0] < a[m][0]) m = r;
        if (m === i) break;
        [a[i], a[m]] = [a[m], a[i]];
        i = m;
      }
    }
    return top;
  }
}

export function deliveryTimes(n: number, roads: number[][], start: number): number[] {
  const adj: [number, number][][] = Array.from({ length: n }, () => []);
  for (const [a, b, w] of roads) {
    adj[a].push([b, w]);
    adj[b].push([a, w]);
  }
  const dist = new Array<number>(n).fill(-1);
  dist[start] = 0;
  const heap = new PairHeap();
  heap.push([0, start]);
  while (heap.size > 0) {
    const [d, v] = heap.pop();
    for (const [u, w] of adj[v]) {
      if (dist[u] === -1) {
        dist[u] = d + w;
        heap.push([d + w, u]);
      }
    }
  }
  return dist;
}
