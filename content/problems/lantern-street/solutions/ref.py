# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def brightest_street(root: TreeNode | None) -> int:
    def best(node):
        if node is None:
            return 0, 0
        l_lit, l_dark = best(node.left)
        r_lit, r_dark = best(node.right)
        return node.val + l_dark + r_dark, max(l_lit, l_dark) + max(r_lit, r_dark)

    return max(best(root))
