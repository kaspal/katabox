const RED = true;
const BLACK = false;

class RbNode {
  value: number;
  left: RbNode | null = null;
  right: RbNode | null = null;
  color = RED;

  constructor(value: number) {
    this.value = value;
  }
}

function isRed(node: RbNode | null): boolean {
  return node !== null && node.color === RED;
}

function rotateLeft(h: RbNode): RbNode {
  const x = h.right!;
  h.right = x.left; // @step rotate-left
  x.left = h;
  x.color = h.color;
  h.color = RED;
  return x;
}

function rotateRight(h: RbNode): RbNode {
  const x = h.left!;
  h.left = x.right; // @step rotate-right
  x.right = h;
  x.color = h.color;
  h.color = RED;
  return x;
}

function flipColors(h: RbNode): void {
  h.color = RED; // @step flip
  h.left!.color = BLACK;
  h.right!.color = BLACK;
}

function insert(h: RbNode | null, value: number): RbNode {
  if (h === null) {
    return new RbNode(value); // @step place
  }
  if (value < h.value) h.left = insert(h.left, value);
  else h.right = insert(h.right, value);
  if (isRed(h.right) && !isRed(h.left)) h = rotateLeft(h);
  if (isRed(h.left) && isRed(h.left!.left)) h = rotateRight(h);
  if (isRed(h.left) && isRed(h.right)) flipColors(h);
  return h;
}

function preorder(
  node: RbNode | null,
  values: number[],
  colors: number[],
): void {
  if (node !== null) {
    values.push(node.value);
    colors.push(node.color === RED ? 1 : 0);
    preorder(node.left, values, colors);
    preorder(node.right, values, colors);
  }
}

function run(keys: number[]): number[] {
  let root: RbNode | null = null;
  for (const key of keys) {
    root = insert(root, key);
    root.color = BLACK; // @step root-black
  }
  const values: number[] = [];
  const colors: number[] = [];
  preorder(root, values, colors);
  return [...values, ...colors];
}
