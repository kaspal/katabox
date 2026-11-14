class Node:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None


class BST:
    def __init__(self):
        self.root = None

    def insert(self, value):
        parent, node = None, self.root
        while node is not None:
            parent = node
            node = node.left if value < node.value else node.right  # @step descend
        new = Node(value)
        if parent is None:
            self.root = new  # @step root
        elif value < parent.value:
            parent.left = new  # @step attach-left
        else:
            parent.right = new  # @step attach-right

    def find(self, value):
        node = self.root
        while node is not None and node.value != value:
            node = node.left if value < node.value else node.right  # @step search
        return 1 if node is not None else 0  # @step result

    def delete(self, value):
        self.root = self._delete(self.root, value)

    def _delete(self, node, value):
        if node is None:
            return None  # @step missing
        if value < node.value:  # @step visit
            node.left = self._delete(node.left, value)
        elif value > node.value:
            node.right = self._delete(node.right, value)
        elif node.left is None or node.right is None:
            return node.left or node.right  # @step splice
        else:
            succ = node.right
            while succ.left is not None:
                succ = succ.left  # @step successor
            node.value = succ.value  # @step copy
            node.right = self._delete(node.right, succ.value)
        return node


def inorder(node, out):
    if node is not None:
        inorder(node.left, out)
        out.append(node.value)
        inorder(node.right, out)


def run(ops):
    tree, out = BST(), []
    for op in ops:
        if op[0] == "insert":
            tree.insert(op[1])
        elif op[0] == "find":
            out.append(tree.find(op[1]))
        else:
            tree.delete(op[1])
    out.append(0)
    inorder(tree.root, out)
    return out
