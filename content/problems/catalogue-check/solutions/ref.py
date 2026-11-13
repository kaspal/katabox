# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def is_search_tree(root: TreeNode | None) -> bool:
    def ok(node, low, high):
        if node is None:
            return True
        if (low is not None and node.val <= low) or (high is not None and node.val >= high):
            return False
        return ok(node.left, low, node.val) and ok(node.right, node.val, high)

    return ok(root, None, None)
