function cross(o: number[], a: number[], b: number[]): number {
  return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
}

function convexHull(points: number[][]): number[] {
  const unique = new Map(points.map((p) => [`${p[0]},${p[1]}`, p]));
  const pts = [...unique.values()].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  if (pts.length <= 2) return pts.flat();
  const lower: number[][] = [];
  const upper: number[][] = [];
  for (const p of pts) {
    while (
      lower.length >= 2 &&
      cross(lower[lower.length - 2], lower[lower.length - 1], p) <= 0
    ) {
      lower.pop(); // @step drop
    }
    lower.push(p); // @step keep
  }
  for (const p of [...pts].reverse()) {
    while (
      upper.length >= 2 &&
      cross(upper[upper.length - 2], upper[upper.length - 1], p) <= 0
    ) {
      upper.pop(); // @step drop-upper
    }
    upper.push(p); // @step keep-upper
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)].flat();
}
