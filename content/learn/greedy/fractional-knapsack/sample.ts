function gcd(a: number, b: number): number {
  return b === 0 ? a : gcd(b, a % b);
}

function fractional(capacity: number, items: number[][]): number[] {
  const order = items
    .map((_, k) => k)
    .sort(
      (x, y) => items[y][1] / items[y][0] - items[x][1] / items[x][0] || x - y,
    );
  let whole = 0;
  let room = capacity;
  for (const k of order) {
    const [weight, value] = items[k];
    if (weight <= room) {
      whole += value; // @step take
      room -= weight;
    } else {
      const num = value * room; // @step part
      const den = weight;
      const g = gcd(num, den);
      return [whole + Math.floor(num / den), (num % den) / g, den / g];
    }
  }
  return [whole, 0, 1];
}
