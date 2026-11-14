RED, BLACK = True, False


class Node:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None
        self.color = RED


def is_red(node):
    return node is not None and node.color == RED


def rotate_left(h):
    x = h.right
    h.right = x.left  # @step rotate-left
    x.left = h
    x.color = h.color
    h.color = RED
    return x


def rotate_right(h):
    x = h.left
    h.left = x.right  # @step rotate-right
    x.right = h
    x.color = h.color
    h.color = RED
    return x


def flip_colors(h):
    h.color = RED  # @step flip
    h.left.color = BLACK
    h.right.color = BLACK


def insert(h, value):
    if h is None:
        return Node(value)  # @step place
    if value < h.value:
        h.left = insert(h.left, value)
    else:
        h.right = insert(h.right, value)
    if is_red(h.right) and not is_red(h.left):
        h = rotate_left(h)
    if is_red(h.left) and is_red(h.left.left):
        h = rotate_right(h)
    if is_red(h.left) and is_red(h.right):
        flip_colors(h)
    return h


def preorder(node, values, colors):
    if node is not None:
        values.append(node.value)
        colors.append(1 if node.color == RED else 0)
        preorder(node.left, values, colors)
        preorder(node.right, values, colors)


def run(keys):
    root = None
    for key in keys:
        root = insert(root, key)
        root.color = BLACK  # @step root-black
    values, colors = [], []
    preorder(root, values, colors)
    return values + colors
