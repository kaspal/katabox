# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def fullest_level(root: TreeNode | None) -> int:
    # Obviously correct: a recursive walk adds each value to its depth's total.
    totals = {}

    def walk(node, depth):
        if node is None:
            return
        totals[depth] = totals.get(depth, 0) + node.val
        walk(node.left, depth + 1)
        walk(node.right, depth + 1)

    walk(root, 1)
    if not totals:
        return 0
    best = max(totals.values())
    return min(d for d, t in totals.items() if t == best)
