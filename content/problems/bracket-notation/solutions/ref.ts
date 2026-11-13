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

export function bracketNotation(root: TreeNode | null): string {
  const parts: string[] = [];
  const write = (node: TreeNode): void => {
    parts.push(String(node.val));
    if (node.left !== null || node.right !== null) {
      parts.push("(");
      if (node.left !== null) write(node.left);
      parts.push(")");
    }
    if (node.right !== null) {
      parts.push("(");
      write(node.right);
      parts.push(")");
    }
  };
  if (root !== null) write(root);
  return parts.join("");
}
