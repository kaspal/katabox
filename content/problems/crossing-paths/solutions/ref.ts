export function pathsCross(pairs: number[][]): boolean[] {
  const orient = (ax: number, ay: number, bx: number, by: number, cx: number, cy: number) =>
    Math.sign((bx - ax) * (cy - ay) - (by - ay) * (cx - ax));
  const within = (ax: number, ay: number, bx: number, by: number, px: number, py: number) =>
    Math.min(ax, bx) <= px && px <= Math.max(ax, bx) && Math.min(ay, by) <= py && py <= Math.max(ay, by);
  return pairs.map(([x1, y1, x2, y2, x3, y3, x4, y4]) => {
    const d1 = orient(x1, y1, x2, y2, x3, y3);
    const d2 = orient(x1, y1, x2, y2, x4, y4);
    const d3 = orient(x3, y3, x4, y4, x1, y1);
    const d4 = orient(x3, y3, x4, y4, x2, y2);
    if (d1 * d2 < 0 && d3 * d4 < 0) return true;
    return (
      (d1 === 0 && within(x1, y1, x2, y2, x3, y3)) ||
      (d2 === 0 && within(x1, y1, x2, y2, x4, y4)) ||
      (d3 === 0 && within(x3, y3, x4, y4, x1, y1)) ||
      (d4 === 0 && within(x3, y3, x4, y4, x2, y2))
    );
  });
}
