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

export function brightestStreet(root: TreeNode | null): number {
  const best = (node: TreeNode | null): [number, number] => {
    if (node === null) return [0, 0];
    const [lLit, lDark] = best(node.left);
    const [rLit, rDark] = best(node.right);
    return [node.val + lDark + rDark, Math.max(lLit, lDark) + Math.max(rLit, rDark)];
  };
  return Math.max(...best(root));
}
