const MAX_KEYS = 2;

class BNode {
  keys: number[];
  children: BNode[];

  constructor(keys: number[], children: BNode[] = []) {
    this.keys = keys;
    this.children = children;
  }
}

// Insert key below node; return [left, middle, right] if node split.
function insert(node: BNode, key: number): [BNode, number, BNode] | null {
  if (node.children.length === 0) {
    node.keys.push(key);
    node.keys.sort((x, y) => x - y); // @step add-key
  } else {
    let i = 0;
    while (i < node.keys.length && key >= node.keys[i]) i++;
    const split = insert(node.children[i], key); // @step descend
    if (split !== null) {
      const [left, middle, right] = split;
      node.keys.splice(i, 0, middle); // @step absorb
      node.children.splice(i, 1, left, right);
    }
  }
  if (node.keys.length > MAX_KEYS) {
    const mid = Math.floor(node.keys.length / 2);
    const left = new BNode(
      node.keys.slice(0, mid),
      node.children.slice(0, mid + 1),
    );
    const right = new BNode(
      node.keys.slice(mid + 1),
      node.children.slice(mid + 1),
    );
    return [left, node.keys[mid], right]; // @step split
  }
  return null;
}

function run(values: number[]): number[] {
  let root = new BNode([]);
  for (const value of values) {
    const split = insert(root, value);
    if (split !== null) {
      const [left, middle, right] = split;
      root = new BNode([middle], [left, right]); // @step new-root
    }
  }
  const out: number[] = [];
  let level = [root];
  while (level.length > 0) {
    const next: BNode[] = [];
    for (const node of level) {
      out.push(...node.keys, 0);
      next.push(...node.children);
    }
    level = next;
  }
  return out;
}
