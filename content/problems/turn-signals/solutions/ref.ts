export function turnSignals(routes: number[][]): string[] {
  return routes.map(([ax, ay, bx, by, cx, cy]) => {
    const cross = (bx - ax) * (cy - by) - (by - ay) * (cx - bx);
    return cross > 0 ? "left" : cross < 0 ? "right" : "straight";
  });
}
