function windowMax(a: number[], k: number): number[] {
  const dq: number[] = [];
  const out: number[] = [];
  for (let i = 0; i < a.length; i++) {
    while (dq.length > 0 && a[dq[dq.length - 1]] <= a[i]) {
      dq.pop(); // @step pop-back
    }
    dq.push(i); // @step push-back
    if (dq[0] <= i - k) {
      dq.shift(); // @step pop-front
    }
    if (i >= k - 1) {
      out.push(a[dq[0]]); // @step emit
    }
  }
  return out;
}
