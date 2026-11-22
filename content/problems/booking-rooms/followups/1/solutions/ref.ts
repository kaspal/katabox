class Heap<T> {
  private a: T[] = [];
  private less: (x: T, y: T) => boolean;
  constructor(less: (x: T, y: T) => boolean) {
    this.less = less;
  }
  get size() {
    return this.a.length;
  }
  peek(): T {
    return this.a[0];
  }
  push(x: T) {
    const a = this.a;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.less(a[i], a[p])) break;
      [a[p], a[i]] = [a[i], a[p]];
      i = p;
    }
  }
  pop(): T {
    const a = this.a;
    const top = a[0];
    const last = a.pop()!;
    if (a.length > 0) {
      a[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < a.length && this.less(a[l], a[m])) m = l;
        if (r < a.length && this.less(a[r], a[m])) m = r;
        if (m === i) break;
        [a[m], a[i]] = [a[i], a[m]];
        i = m;
      }
    }
    return top;
  }
}

export function assignRooms(meetings: number[][]): number[] {
  const order = meetings.map((_, i) => i).sort((x, y) => meetings[x][0] - meetings[y][0] || x - y);
  const free = new Heap<number>((x, y) => x < y);
  const busy = new Heap<[number, number]>((x, y) => x[0] < y[0] || (x[0] === y[0] && x[1] < y[1]));
  let rooms = 0;
  const out = new Array<number>(meetings.length).fill(0);
  for (const i of order) {
    const [s, e] = meetings[i];
    while (busy.size > 0 && busy.peek()[0] <= s) free.push(busy.pop()[1]);
    const room = free.size > 0 ? free.pop() : ++rooms;
    out[i] = room;
    busy.push([e, room]);
  }
  return out;
}
