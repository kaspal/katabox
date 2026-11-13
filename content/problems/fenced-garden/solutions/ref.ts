export function whereIsPoint(fence: number[][], points: number[][]): string[] {
  const n = fence.length;
  return points.map(([px, py]) => {
    let inside = false;
    for (let i = 0; i < n; i++) {
      const [ax, ay] = fence[i];
      const [bx, by] = fence[(i + 1) % n];
      if (
        (bx - ax) * (py - ay) - (by - ay) * (px - ax) === 0 &&
        Math.min(ax, bx) <= px && px <= Math.max(ax, bx) &&
        Math.min(ay, by) <= py && py <= Math.max(ay, by)
      ) {
        return "boundary";
      }
      if (ay > py !== by > py) {
        const t = (py - ay) * (bx - ax) - (px - ax) * (by - ay);
        if (t > 0 === by > ay) inside = !inside;
      }
    }
    return inside ? "inside" : "outside";
  });
}
