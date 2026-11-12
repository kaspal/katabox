export function holds(ops: string[], args: number[][]): number[] {
  // Per book: a queue (array + head index) of [member, ticket] entries.
  const line = new Map<number, { items: [number, number][]; head: number }>();
  const waiting = new Map<string, number>();
  let tickets = 0;
  const reports: number[] = [];
  ops.forEach((op, i) => {
    const [book, member] = args[i];
    if (op === "hold") {
      tickets++;
      waiting.set(`${book}:${member}`, tickets);
      let q = line.get(book);
      if (!q) line.set(book, (q = { items: [], head: 0 }));
      q.items.push([member, tickets]);
    } else if (op === "cancel") {
      waiting.delete(`${book}:${member}`);
    } else {
      const q = line.get(book);
      let got = -1;
      while (q && q.head < q.items.length) {
        const [m, t] = q.items[q.head++];
        const key = `${book}:${m}`;
        if (waiting.get(key) === t) {
          waiting.delete(key);
          got = m;
          break;
        }
      }
      reports.push(got);
    }
  });
  return reports;
}
