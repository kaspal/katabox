class TreeNode {
  value: number;
  left: TreeNode | null = null;
  right: TreeNode | null = null;

  constructor(value: number) {
    this.value = value;
  }
}

class BST {
  root: TreeNode | null = null;

  insert(value: number): void {
    let parent: TreeNode | null = null;
    let node = this.root;
    while (node !== null) {
      parent = node;
      node = value < node.value ? node.left : node.right; // @step descend
    }
    const created = new TreeNode(value);
    if (parent === null) {
      this.root = created; // @step root
    } else if (value < parent.value) {
      parent.left = created; // @step attach-left
    } else {
      parent.right = created; // @step attach-right
    }
  }

  find(value: number): number {
    let node = this.root;
    while (node !== null && node.value !== value) {
      node = value < node.value ? node.left : node.right; // @step search
    }
    return node !== null ? 1 : 0; // @step result
  }

  delete(value: number): void {
    this.root = this.remove(this.root, value);
  }

  remove(node: TreeNode | null, value: number): TreeNode | null {
    if (node === null) {
      return null; // @step missing
    }
    if (value < node.value) { // @step visit
      node.left = this.remove(node.left, value);
    } else if (value > node.value) {
      node.right = this.remove(node.right, value);
    } else if (node.left === null || node.right === null) {
      return node.left ?? node.right; // @step splice
    } else {
      let succ = node.right;
      while (succ.left !== null) {
        succ = succ.left; // @step successor
      }
      node.value = succ.value; // @step copy
      node.right = this.remove(node.right, succ.value);
    }
    return node;
  }
}

function inorder(node: TreeNode | null, out: number[]): void {
  if (node !== null) {
    inorder(node.left, out);
    out.push(node.value);
    inorder(node.right, out);
  }
}

function run(ops: (string | number)[][]): number[] {
  const tree = new BST();
  const out: number[] = [];
  for (const op of ops) {
    if (op[0] === "insert") tree.insert(op[1] as number);
    else if (op[0] === "find") out.push(tree.find(op[1] as number));
    else tree.delete(op[1] as number);
  }
  out.push(0);
  inorder(tree.root, out);
  return out;
}
