# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def trim_hedge(root: TreeNode | None) -> TreeNode | None:
    def trim(node):
        if node is None:
            return None, 0
        node.left, a = trim(node.left)
        node.right, b = trim(node.right)
        total = node.val + a + b
        if total < 0:
            return None, 0
        return node, total

    return trim(root)[0]
