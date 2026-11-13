# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def is_search_tree(root: TreeNode | None) -> bool:
    # Obviously correct: compare every node with every value in its two subtrees.
    def values(node):
        return [] if node is None else values(node.left) + [node.val] + values(node.right)

    def check(node):
        if node is None:
            return True
        return (
            all(v < node.val for v in values(node.left))
            and all(v > node.val for v in values(node.right))
            and check(node.left)
            and check(node.right)
        )

    return check(root)
