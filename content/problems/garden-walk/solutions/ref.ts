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

export function longestWalk(root: TreeNode | null): number {
  let best = 0;
  const depth = (node: TreeNode | null): number => {
    if (node === null) return 0;
    const a = depth(node.left);
    const b = depth(node.right);
    best = Math.max(best, a + b);
    return 1 + Math.max(a, b);
  };
  depth(root);
  return best;
}
