class MinHeap {
  private a: [number, number][] = [];
  get size() {
    return this.a.length;
  }
  push(x: [number, number]) {
    const a = this.a;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p][0] <= a[i][0]) break;
      [a[p], a[i]] = [a[i], a[p]];
      i = p;
    }
  }
  pop(): [number, number] {
    const a = this.a;
    const top = a[0];
    const last = a.pop()!;
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
        [a[m], a[i]] = [a[i], a[m]];
        i = m;
      }
    }
    return top;
  }
}

export function cheapestWithPass(n: number, roads: number[][], start: number, goal: number): number {
  const adj: [number, number][][] = Array.from({ length: n }, () => []);
  for (const [a, b, t] of roads) {
    adj[a].push([b, t]);
    adj[b].push([a, t]);
  }
  const dist = new Array<number>(2 * n).fill(Infinity); // state = station * 2 + used
  dist[start * 2] = 0;
  const heap = new MinHeap();
  heap.push([0, start * 2]);
  while (heap.size > 0) {
    const [d, s] = heap.pop();
    if (d > dist[s]) continue;
    const u = s >> 1;
    const used = s & 1;
    for (const [v, t] of adj[u]) {
      if (d + t < dist[v * 2 + used]) {
        dist[v * 2 + used] = d + t;
        heap.push([d + t, v * 2 + used]);
      }
      if (used === 0 && d < dist[v * 2 + 1]) {
        dist[v * 2 + 1] = d;
        heap.push([d, v * 2 + 1]);
      }
    }
  }
  const best = Math.min(dist[goal * 2], dist[goal * 2 + 1]);
  return best === Infinity ? -1 : best;
}
