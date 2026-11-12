export function cheapestRoute(n: number, roads: number[][], start: number, goal: number): number {
  const adj: [number, number][][] = Array.from({ length: n }, () => []);
  for (const [a, b, toll] of roads) {
    adj[a].push([b, toll]);
    adj[b].push([a, toll]);
  }
  const dist = new Array<number>(n).fill(Infinity);
  dist[start] = 0;
  // Binary min-heap of [cost, town].
  const heap: [number, number][] = [[0, start]];
  const push = (e: [number, number]) => {
    heap.push(e);
    let i = heap.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (heap[p][0] <= heap[i][0]) break;
      [heap[p], heap[i]] = [heap[i], heap[p]];
      i = p;
    }
  };
  const pop = (): [number, number] => {
    const top = heap[0];
    const last = heap.pop()!;
    if (heap.length > 0) {
      heap[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < heap.length && heap[l][0] < heap[m][0]) m = l;
        if (r < heap.length && heap[r][0] < heap[m][0]) m = r;
        if (m === i) break;
        [heap[m], heap[i]] = [heap[i], heap[m]];
        i = m;
      }
    }
    return top;
  };
  while (heap.length > 0) {
    const [d, t] = pop();
    if (d > dist[t]) continue;
    if (t === goal) return d;
    for (const [u, toll] of adj[t]) {
      if (d + toll < dist[u]) {
        dist[u] = d + toll;
        push([d + toll, u]);
      }
    }
  }
  return -1;
}
