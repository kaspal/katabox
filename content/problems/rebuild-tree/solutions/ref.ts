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

export function rebuildTree(preorder: number[], inorder: number[]): TreeNode | null {
  const pos = new Map<number, number>();
  inorder.forEach((v, i) => pos.set(v, i));
  const build = (ps: number, ins: number, length: number): TreeNode | null => {
    if (length === 0) return null;
    const root = new TreeNode(preorder[ps]);
    const left = pos.get(root.val)! - ins;
    root.left = build(ps + 1, ins, left);
    root.right = build(ps + 1 + left, ins + left + 1, length - left - 1);
    return root;
  };
  return build(0, 0, preorder.length);
}
