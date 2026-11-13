export function recentPages(capacity: number, ops: string[], args: number[][]): number[] {
  // A Map keeps insertion order: delete and re-insert to mark a page as used.
  const cache = new Map<number, number>();
  const out: number[] = [];
  ops.forEach((op, i) => {
    const page = args[i][0];
    if (op === "get") {
      const value = cache.get(page);
      if (value === undefined) {
        out.push(-1);
      } else {
        cache.delete(page);
        cache.set(page, value);
        out.push(value);
      }
    } else {
      cache.delete(page);
      cache.set(page, args[i][1]);
      if (cache.size > capacity) cache.delete(cache.keys().next().value!);
    }
  });
  return out;
}
