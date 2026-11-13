export function bestXorPair(codes: number[]): number {
  // Binary trie: children[2 * node + bit], 0 means "no child".
  const children = new Int32Array(2 * (codes.length * 20 + 1));
  let size = 1;
  let best = 0;
  codes.forEach((x, i) => {
    if (i > 0) {
      let node = 0;
      let value = 0;
      for (let bit = 19; bit >= 0; bit--) {
        const b = (x >> bit) & 1;
        if (children[2 * node + (b ^ 1)] !== 0) {
          value |= 1 << bit;
          node = children[2 * node + (b ^ 1)];
        } else {
          node = children[2 * node + b];
        }
      }
      best = Math.max(best, value);
    }
    let node = 0;
    for (let bit = 19; bit >= 0; bit--) {
      const b = (x >> bit) & 1;
      if (children[2 * node + b] === 0) children[2 * node + b] = size++;
      node = children[2 * node + b];
    }
  });
  return best;
}
