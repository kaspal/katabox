# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def tree_height(root: TreeNode | None) -> int:
    if root is None:
        return 0
    return 1 + max(tree_height(root.left), tree_height(root.right))
