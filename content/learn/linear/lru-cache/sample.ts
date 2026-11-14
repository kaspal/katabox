class LRUCache {
  capacity: number;
  items = new Map<number, number>();

  constructor(capacity: number) {
    this.capacity = capacity;
  }

  get(key: number): number {
    const value = this.items.get(key);
    if (value === undefined) {
      return -1; // @step miss
    }
    this.items.delete(key); // @step hit
    this.items.set(key, value);
    return value;
  }

  put(key: number, value: number): void {
    if (this.items.has(key)) {
      this.items.delete(key); // @step refresh
    }
    this.items.set(key, value); // @step put
    if (this.items.size > this.capacity) {
      this.items.delete(this.items.keys().next().value!); // @step evict
    }
  }
}

function run(capacity: number, ops: (string | number)[][]): number[] {
  const cache = new LRUCache(capacity);
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "get") out.push(cache.get(op[1] as number));
    else cache.put(op[1] as number, op[2] as number);
  }
  return out;
}
