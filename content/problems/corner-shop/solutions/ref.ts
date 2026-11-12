export function shop(ops: string[], args: number[][]): number[] {
  const shelf = new Map<number, number>();
  const reports: number[] = [];
  ops.forEach((op, i) => {
    const [item, qty] = args[i];
    const have = shelf.get(item) ?? 0;
    if (op === "stock") {
      shelf.set(item, have + qty);
    } else if (op === "sell") {
      const sold = Math.min(qty, have);
      shelf.set(item, have - sold);
      reports.push(sold);
    } else {
      reports.push(have);
    }
  });
  return reports;
}
