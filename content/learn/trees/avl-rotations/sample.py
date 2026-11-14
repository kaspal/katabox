class Node:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None
        self.height = 1


def height(node):
    return node.height if node is not None else 0


def update(node):
    node.height = 1 + max(height(node.left), height(node.right))


def rotate_right(y):
    x = y.left
    y.left = x.right  # @step rotate-right
    x.right = y
    update(y)
    update(x)
    return x


def rotate_left(x):
    y = x.right
    x.right = y.left  # @step rotate-left
    y.left = x
    update(x)
    update(y)
    return y


def insert(node, value):
    if node is None:
        return Node(value)  # @step place
    if value < node.value:
        node.left = insert(node.left, value)
    else:
        node.right = insert(node.right, value)
    update(node)
    balance = height(node.left) - height(node.right)  # @step balance
    if balance > 1:
        if value >= node.left.value:
            node.left = rotate_left(node.left)
        return rotate_right(node)
    if balance < -1:
        if value < node.right.value:
            node.right = rotate_right(node.right)
        return rotate_left(node)
    return node


def preorder(node, out):
    if node is not None:
        out.append(node.value)
        preorder(node.left, out)
        preorder(node.right, out)


def run(values):
    root = None
    for value in values:
        root = insert(root, value)
    out = []
    preorder(root, out)
    return out
