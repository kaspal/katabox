# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def rebuild_tree(preorder: list[int], inorder: list[int]) -> TreeNode | None:
    # Obviously correct: slice the lists by the root's position, recursively.
    if not preorder:
        return None
    root = preorder[0]
    i = inorder.index(root)
    return TreeNode(root, rebuild_tree(preorder[1 : 1 + i], inorder[:i]), rebuild_tree(preorder[1 + i :], inorder[i + 1 :]))
