# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def trim_hedge(root: TreeNode | None) -> TreeNode | None:
    # Obviously correct: copy the tree, then repeatedly cut the deepest-first branches
    # whose current total (recomputed from scratch) is negative.
    def copy(node):
        if node is None:
            return None
        return TreeNode(node.val, copy(node.left), copy(node.right))

    def total(node):
        return 0 if node is None else node.val + total(node.left) + total(node.right)

    def post_order(node, out):
        if node is not None:
            post_order(node.left, out)
            post_order(node.right, out)
            out.append(node)
        return out

    hedge = copy(root)
    for node in post_order(hedge, []):
        if node.left is not None and total(node.left) < 0:
            node.left = None
        if node.right is not None and total(node.right) < 0:
            node.right = None
    if hedge is not None and total(hedge) < 0:
        return None
    return hedge
