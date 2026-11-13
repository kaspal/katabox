# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def window_view(root: TreeNode | None) -> list[int]:
    seen = []
    level = [root] if root is not None else []
    while level:
        seen.append(level[-1].val)
        level = [c for node in level for c in (node.left, node.right) if c is not None]
    return seen
