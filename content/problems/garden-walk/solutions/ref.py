# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def longest_walk(root: TreeNode | None) -> int:
    best = 0

    def depth(node):
        nonlocal best
        if node is None:
            return 0
        a, b = depth(node.left), depth(node.right)
        best = max(best, a + b)
        return 1 + max(a, b)

    depth(root)
    return best
