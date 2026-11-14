class DSU {
  parent: number[];
  size: number[];

  constructor(n: number) {
    this.parent = Array.from({ length: n }, (_, k) => k);
    this.size = new Array<number>(n).fill(1);
  }

  find(x: number): number {
    let root = x;
    while (this.parent[root] !== root) {
      root = this.parent[root]; // @step climb
    }
    while (this.parent[x] !== root) {
      [this.parent[x], x] = [root, this.parent[x]]; // @step compress
    }
    return root;
  }

  union(a: number, b: number): boolean {
    let ra = this.find(a);
    let rb = this.find(b);
    if (ra === rb) {
      return false; // @step same
    }
    if (this.size[ra] < this.size[rb]) [ra, rb] = [rb, ra];
    this.parent[rb] = ra; // @step link
    this.size[ra] += this.size[rb];
    return true;
  }
}

function run(n: number, ops: (string | number)[][]): number[] {
  const dsu = new DSU(n);
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "union") dsu.union(op[1] as number, op[2] as number);
    else out.push(dsu.find(op[1] as number));
  }
  let roots = 0;
  for (let x = 0; x < n; x++) if (dsu.parent[x] === x) roots++;
  out.push(roots);
  return out;
}
