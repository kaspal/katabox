# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def richest_trail(root: TreeNode | None) -> int:
    best = float("-inf")

    def gain(node):
        nonlocal best
        if node is None:
            return 0
        a, b = max(0, gain(node.left)), max(0, gain(node.right))
        best = max(best, node.val + a + b)
        return node.val + max(a, b)

    gain(root)
    return int(best)
