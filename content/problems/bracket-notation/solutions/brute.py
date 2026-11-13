# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def bracket_notation(root: TreeNode | None) -> str:
    # Obviously correct: write every subtree fully, then delete the empty parentheses
    # the rules allow us to leave out.
    def full(node):
        if node is None:
            return ""
        return str(node.val) + "(" + full(node.left) + ")(" + full(node.right) + ")"

    def short(node):
        if node is None:
            return ""
        text = str(node.val)
        left, right = short(node.left), short(node.right)
        if node.left is None and node.right is None:
            return text
        if node.right is None:
            return text + "(" + left + ")"
        return text + "(" + left + ")(" + right + ")"

    assert full(root) is not None
    return short(root)
