from collections import deque


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


def level_order(root):
    out, queue = [], deque([root])
    while queue:
        node = queue.popleft()  # @step visit
        out.append(node.value)
        for child in (node.left, node.right):
            if child is not None:
                queue.append(child)  # @step enqueue
    return out


def run(values):
    root = None
    for value in values:
        root = insert(root, value)
    return level_order(root)
