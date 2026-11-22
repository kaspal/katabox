function orient(a: number[], b: number[], c: number[]): number {
  return Math.sign(
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]),
  );
}

function onSegment(a: number[], b: number[], c: number[]): boolean {
  return (
    Math.min(a[0], b[0]) <= c[0] &&
    c[0] <= Math.max(a[0], b[0]) &&
    Math.min(a[1], b[1]) <= c[1] &&
    c[1] <= Math.max(a[1], b[1])
  );
}

function intersects(
  p1: number[],
  p2: number[],
  p3: number[],
  p4: number[],
): number[] {
  const checks = [[p3, p4, p1], [p3, p4, p2], [p1, p2, p3], [p1, p2, p4]];
  const signs: number[] = [];
  for (const [a, b, c] of checks) {
    signs.push(orient(a, b, c)); // @step side
  }
  if (signs[0] * signs[1] < 0 && signs[2] * signs[3] < 0) {
    return [1, ...signs]; // @step cross
  }
  for (let k = 0; k < 4; k++) {
    const [a, b, c] = checks[k];
    if (signs[k] === 0 && onSegment(a, b, c)) {
      return [1, ...signs]; // @step touch
    }
  }
  return [0, ...signs];
}
