export function twinsAfterChanges(a: string, b: string, positions: number[], letters: string): boolean[] {
  const diff = new Array<number>(26).fill(0);
  for (const c of a) diff[c.charCodeAt(0) - 97]++;
  for (const c of b) diff[c.charCodeAt(0) - 97]--;
  let off = diff.filter((d) => d !== 0).length;
  const tiles = b.split("");
  const out: boolean[] = [];
  const bump = (letter: string, step: number) => {
    const k = letter.charCodeAt(0) - 97;
    const before = diff[k] !== 0 ? 1 : 0;
    diff[k] += step;
    off += (diff[k] !== 0 ? 1 : 0) - before;
  };
  for (let i = 0; i < positions.length; i++) {
    const p = positions[i];
    bump(tiles[p], 1);
    bump(letters[i], -1);
    tiles[p] = letters[i];
    out.push(off === 0);
  }
  return out;
}
