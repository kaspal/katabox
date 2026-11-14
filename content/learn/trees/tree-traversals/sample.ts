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

function preorder(node: TreeNode | null, out: number[]): void {
  if (node !== null) {
    out.push(node.value); // @step pre
    preorder(node.left, out);
    preorder(node.right, out);
  }
}

function inorder(node: TreeNode | null, out: number[]): void {
  if (node !== null) {
    inorder(node.left, out);
    out.push(node.value); // @step in
    inorder(node.right, out);
  }
}

function postorder(node: TreeNode | null, out: number[]): void {
  if (node !== null) {
    postorder(node.left, out);
    postorder(node.right, out);
    out.push(node.value); // @step post
  }
}

function run(values: number[]): number[] {
  let root: TreeNode | null = null;
  for (const value of values) root = insert(root, value);
  const out: number[] = [];
  preorder(root, out);
  inorder(root, out);
  postorder(root, out);
  return out;
}
