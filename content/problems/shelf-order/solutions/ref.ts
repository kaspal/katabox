type Chunk = [number, number, string];

function chunkKey(label: string): Chunk[] {
  const key: Chunk[] = [];
  for (const chunk of label.match(/[0-9]+|[a-z]+/g) ?? []) {
    if (chunk[0] >= "0" && chunk[0] <= "9") {
      const digits = chunk.replace(/^0+/, "");
      key.push([0, digits.length, digits]);
    } else {
      key.push([1, 0, chunk]);
    }
  }
  return key;
}

function compareKeys(a: Chunk[], b: Chunk[]): number {
  for (let i = 0; i < Math.min(a.length, b.length); i++) {
    const [ka, la, sa] = a[i];
    const [kb, lb, sb] = b[i];
    if (ka !== kb) return ka - kb;
    if (la !== lb) return la - lb;
    if (sa !== sb) return sa < sb ? -1 : 1;
  }
  return a.length - b.length;
}

export function naturalOrder(labels: string[]): string[] {
  const keyed = labels.map((label) => ({ label, key: chunkKey(label) }));
  keyed.sort((x, y) => compareKeys(x.key, y.key)); // Array.prototype.sort is stable
  return keyed.map((x) => x.label);
}
