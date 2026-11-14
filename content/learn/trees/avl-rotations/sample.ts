class AvlNode {
  value: number;
  left: AvlNode | null = null;
  right: AvlNode | null = null;
  height = 1;

  constructor(value: number) {
    this.value = value;
  }
}

function height(node: AvlNode | null): number {
  return node !== null ? node.height : 0;
}

function update(node: AvlNode): void {
  node.height = 1 + Math.max(height(node.left), height(node.right));
}

function rotateRight(y: AvlNode): AvlNode {
  const x = y.left!;
  y.left = x.right; // @step rotate-right
  x.right = y;
  update(y);
  update(x);
  return x;
}

function rotateLeft(x: AvlNode): AvlNode {
  const y = x.right!;
  x.right = y.left; // @step rotate-left
  y.left = x;
  update(x);
  update(y);
  return y;
}

function insert(node: AvlNode | null, value: number): AvlNode {
  if (node === null) {
    return new AvlNode(value); // @step place
  }
  if (value < node.value) node.left = insert(node.left, value);
  else node.right = insert(node.right, value);
  update(node);
  const balance = height(node.left) - height(node.right); // @step balance
  if (balance > 1) {
    if (value >= node.left!.value) node.left = rotateLeft(node.left!);
    return rotateRight(node);
  }
  if (balance < -1) {
    if (value < node.right!.value) node.right = rotateRight(node.right!);
    return rotateLeft(node);
  }
  return node;
}

function preorder(node: AvlNode | null, out: number[]): void {
  if (node !== null) {
    out.push(node.value);
    preorder(node.left, out);
    preorder(node.right, out);
  }
}

function run(values: number[]): number[] {
  let root: AvlNode | null = null;
  for (const value of values) root = insert(root, value);
  const out: number[] = [];
  preorder(root, out);
  return out;
}
