# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def tree_height(root: TreeNode | None) -> int:
    # Obviously correct: walk level by level and count the levels.
    level = [root] if root is not None else []
    height = 0
    while level:
        height += 1
        level = [c for n in level for c in (n.left, n.right) if c is not None]
    return height
