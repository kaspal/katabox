// The queue keeps (frequency, node) pairs sorted, smallest first.
function huffman(freqs: number[]): number[] {
  let queue = freqs
    .map((f, k) => [f, k])
    .sort((x, y) => x[0] - y[0] || x[1] - y[1]);
  const parent = new Map<number, number>();
  let made = freqs.length;
  while (queue.length > 1) {
    const [fa, a] = queue[0];
    const [fb, b] = queue[1];
    queue = queue.slice(2);
    parent.set(a, made); // @step merge
    parent.set(b, made);
    queue.push([fa + fb, made]);
    queue.sort((x, y) => x[0] - y[0] || x[1] - y[1]);
    made++;
  }
  const lengths = freqs.map((_, k) => {
    let depth = 0;
    for (let node = k; parent.has(node); node = parent.get(node)!) depth++;
    return depth;
  });
  const bits = freqs.reduce((sum, f, k) => sum + f * lengths[k], 0);
  return [bits, ...lengths];
}
