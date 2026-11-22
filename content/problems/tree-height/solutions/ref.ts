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

export function treeHeight(root: TreeNode | null): number {
  if (root === null) return 0;
  return 1 + Math.max(treeHeight(root.left), treeHeight(root.right));
}
