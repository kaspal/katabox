# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def kth_smallest(root: TreeNode | None, k: int) -> int:
    # Obviously correct: collect every value, sort, index.
    values, todo = [], [root]
    while todo:
        node = todo.pop()
        if node is not None:
            values.append(node.val)
            todo += [node.left, node.right]
    return sorted(values)[k - 1]
