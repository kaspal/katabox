class TreeNode {
  value: number;
  left: TreeNode | null = null;
  right: TreeNode | null = null;

  constructor(value: number) {
    this.value = value;
  }
}

function insert(root: TreeNode | null, value: number): TreeNode {
  if (root === null) return new TreeNode(value);
  if (value < root.value) root.left = insert(root.left, value);
  else root.right = insert(root.right, value);
  return root;
}

function levelOrder(root: TreeNode): number[] {
  const out: number[] = [];
  const queue: TreeNode[] = [root];
  while (queue.length > 0) {
    const node = queue.shift()!; // @step visit
    out.push(node.value);
    for (const child of [node.left, node.right]) {
      if (child !== null) {
        queue.push(child); // @step enqueue
      }
    }
  }
  return out;
}

function run(values: number[]): number[] {
  let root: TreeNode | null = null;
  for (const value of values) root = insert(root, value);
  return levelOrder(root!);
}
