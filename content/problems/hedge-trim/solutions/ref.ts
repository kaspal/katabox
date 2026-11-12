// Provided by KataBox: the judge builds the input trees with this class.
export class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val = 0, left: TreeNode | null = null, right: TreeNode | null = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

export function trimHedge(root: TreeNode | null): TreeNode | null {
  const trim = (node: TreeNode | null): [TreeNode | null, number] => {
    if (node === null) return [null, 0];
    const [left, a] = trim(node.left);
    const [right, b] = trim(node.right);
    const total = node.val + a + b;
    if (total < 0) return [null, 0];
    node.left = left;
    node.right = right;
    return [node, total];
  };
  return trim(root)[0];
}
