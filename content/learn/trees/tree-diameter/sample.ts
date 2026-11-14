function farthest(adj: number[][], start: number): [number, number] {
  const dist = new Array<number>(adj.length).fill(-1);
  dist[start] = 0;
  const queue = [start];
  let best = start;
  while (queue.length > 0) {
    const node = queue.shift()!; // @step visit
    if (dist[node] > dist[best]) best = node;
    for (const nxt of adj[node]) {
      if (dist[nxt] === -1) {
        dist[nxt] = dist[node] + 1; // @step reach
        queue.push(nxt);
      }
    }
  }
  return [best, dist[best]];
}

function diameter(parents: number[]): number[] {
  const n = parents.length + 1;
  const adj: number[][] = Array.from({ length: n }, () => []);
  parents.forEach((parent, k) => {
    adj[parent].push(k + 1);
    adj[k + 1].push(parent);
  });
  const [end] = farthest(adj, 0); // @step first-bfs
  const [, length] = farthest(adj, end); // @step second-bfs
  return [length];
}
