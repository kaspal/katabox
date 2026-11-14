class Node:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None


def insert(root, value):
    if root is None:
        return Node(value)
    if value < root.value:
        root.left = insert(root.left, value)
    else:
        root.right = insert(root.right, value)
    return root


def preorder(node, out):
    if node is not None:
        out.append(node.value)  # @step pre
        preorder(node.left, out)
        preorder(node.right, out)


def inorder(node, out):
    if node is not None:
        inorder(node.left, out)
        out.append(node.value)  # @step in
        inorder(node.right, out)


def postorder(node, out):
    if node is not None:
        postorder(node.left, out)
        postorder(node.right, out)
        out.append(node.value)  # @step post


def run(values):
    root = None
    for value in values:
        root = insert(root, value)
    out = []
    preorder(root, out)
    inorder(root, out)
    postorder(root, out)
    return out
