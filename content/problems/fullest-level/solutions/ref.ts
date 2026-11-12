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

export function fullestLevel(root: TreeNode | null): number {
  if (root === null) return 0;
  let best = -1;
  let bestLevel = 0;
  let level = 0;
  let queue: TreeNode[] = [root];
  while (queue.length > 0) {
    level++;
    let total = 0;
    const next: TreeNode[] = [];
    for (const node of queue) {
      total += node.val;
      if (node.left !== null) next.push(node.left);
      if (node.right !== null) next.push(node.right);
    }
    if (total > best) {
      best = total;
      bestLevel = level;
    }
    queue = next;
  }
  return bestLevel;
}
