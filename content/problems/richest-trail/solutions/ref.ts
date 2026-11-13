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

export function richestTrail(root: TreeNode | null): number {
  let best = -Infinity;
  const gain = (node: TreeNode | null): number => {
    if (node === null) return 0;
    const a = Math.max(0, gain(node.left));
    const b = Math.max(0, gain(node.right));
    best = Math.max(best, node.val + a + b);
    return node.val + Math.max(a, b);
  };
  gain(root);
  return best;
}
