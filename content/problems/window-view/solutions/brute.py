# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def window_view(root: TreeNode | None) -> list[int]:
    # Obviously correct: a depth-first walk left to right; the last value written for
    # each depth is the rightmost one.
    by_depth = {}

    def walk(node, depth):
        if node is None:
            return
        by_depth[depth] = node.val
        walk(node.left, depth + 1)
        walk(node.right, depth + 1)

    walk(root, 0)
    return [by_depth[d] for d in sorted(by_depth)]
