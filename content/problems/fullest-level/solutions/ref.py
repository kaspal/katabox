# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def fullest_level(root: TreeNode | None) -> int:
    if root is None:
        return 0
    best, best_level, level = -1, 0, 0
    queue = [root]
    while queue:
        level += 1
        total = sum(node.val for node in queue)
        if total > best:
            best, best_level = total, level
        queue = [c for node in queue for c in (node.left, node.right) if c is not None]
    return best_level
