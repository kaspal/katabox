export function criticalBridges(n: number, bridges: number[][]): number[][] {
  const adj: [number, number][][] = Array.from({ length: n }, () => []);
  bridges.forEach(([a, b], idx) => {
    adj[a].push([b, idx]);
    adj[b].push([a, idx]);
  });
  const tin = new Int32Array(n).fill(-1);
  const low = new Int32Array(n);
  let timer = 0;
  const out: number[][] = [];
  for (let s = 0; s < n; s++) {
    if (tin[s] !== -1) continue;
    tin[s] = low[s] = timer++;
    const stack: [number, number, number][] = [[s, 0, -1]];
    while (stack.length > 0) {
      const top = stack[stack.length - 1];
      const v = top[0];
      if (top[1] < adj[v].length) {
        const [u, idx] = adj[v][top[1]++];
        if (idx === top[2]) continue;
        if (tin[u] === -1) {
          tin[u] = low[u] = timer++;
          stack.push([u, 0, idx]);
        } else {
          low[v] = Math.min(low[v], tin[u]);
        }
      } else {
        stack.pop();
        if (stack.length > 0) {
          const p = stack[stack.length - 1][0];
          low[p] = Math.min(low[p], low[v]);
          if (low[v] > tin[p]) out.push([Math.min(p, v), Math.max(p, v)]);
        }
      }
    }
  }
  return out.sort((x, y) => x[0] - y[0] || x[1] - y[1]);
}
