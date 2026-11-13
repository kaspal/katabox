export function throttle(limit: number, window: number, ops: string[], args: number[][]): number[] {
  // Per user: the allowed times, and the index of the oldest one still in the window.
  const logs = new Map<number, { times: number[]; head: number }>();
  const out: number[] = [];
  ops.forEach((op, i) => {
    const [user, time] = args[i];
    let log = logs.get(user);
    if (log === undefined) {
      log = { times: [], head: 0 };
      logs.set(user, log);
    }
    while (log.head < log.times.length && log.times[log.head] <= time - window) log.head++;
    const size = log.times.length - log.head;
    if (op === "count") {
      out.push(size);
    } else if (size < limit) {
      log.times.push(time);
      out.push(1);
    } else {
      out.push(0);
    }
  });
  return out;
}
