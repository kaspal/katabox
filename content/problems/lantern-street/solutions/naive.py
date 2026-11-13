# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def brightest_street(root: TreeNode | None) -> int:
    # Correct but exponential: "lit here and best of the grandchildren" versus "best of
    # the children", recomputed every time.
    def best(node):
        if node is None:
            return 0
        lit = node.val
        for child in (node.left, node.right):
            if child is not None:
                lit += best(child.left) + best(child.right)
        return max(lit, best(node.left) + best(node.right))

    return best(root)
