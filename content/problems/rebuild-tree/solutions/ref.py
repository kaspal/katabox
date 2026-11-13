# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def rebuild_tree(preorder: list[int], inorder: list[int]) -> TreeNode | None:
    pos = {v: i for i, v in enumerate(inorder)}

    def build(ps, ins, length):
        if length == 0:
            return None
        root = TreeNode(preorder[ps])
        left = pos[root.val] - ins
        root.left = build(ps + 1, ins, left)
        root.right = build(ps + 1 + left, ins + left + 1, length - left - 1)
        return root

    return build(0, 0, len(preorder))
