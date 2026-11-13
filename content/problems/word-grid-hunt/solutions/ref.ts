type Trie = { next: Map<string, Trie>; word: string | null };

export function findWords(grid: string[], words: string[]): string[] {
  const root: Trie = { next: new Map(), word: null };
  for (const w of words) {
    let node = root;
    for (const ch of w) {
      let child = node.next.get(ch);
      if (child === undefined) {
        child = { next: new Map(), word: null };
        node.next.set(ch, child);
      }
      node = child;
    }
    node.word = w;
  }
  const rows = grid.length;
  const cols = grid[0].length;
  const used: boolean[][] = grid.map((row) => new Array<boolean>(row.length).fill(false));
  const found = new Set<string>();
  const dfs = (r: number, c: number, node: Trie): void => {
    const child = node.next.get(grid[r][c]);
    if (child === undefined) return;
    if (child.word !== null) found.add(child.word);
    used[r][c] = true;
    for (const [nr, nc] of [[r + 1, c], [r - 1, c], [r, c + 1], [r, c - 1]]) {
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && !used[nr][nc]) dfs(nr, nc, child);
    }
    used[r][c] = false;
  };
  for (let r = 0; r < rows; r++) for (let c = 0; c < cols; c++) dfs(r, c, root);
  return [...found].sort();
}
