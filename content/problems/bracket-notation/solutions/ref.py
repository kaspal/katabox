# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def bracket_notation(root: TreeNode | None) -> str:
    parts = []

    def write(node):
        parts.append(str(node.val))
        if node.left is not None or node.right is not None:
            parts.append("(")
            if node.left is not None:
                write(node.left)
            parts.append(")")
        if node.right is not None:
            parts.append("(")
            write(node.right)
            parts.append(")")

    if root is not None:
        write(root)
    return "".join(parts)
