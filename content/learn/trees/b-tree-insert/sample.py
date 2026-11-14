MAX_KEYS = 2


class Node:
    def __init__(self, keys, children=None):
        self.keys = keys
        self.children = children if children is not None else []


def insert(node, key):
    """Insert key below node; return (left, middle, right) if node split."""
    if not node.children:
        node.keys.append(key)
        node.keys.sort()  # @step add-key
    else:
        i = 0
        while i < len(node.keys) and key >= node.keys[i]:
            i += 1
        split = insert(node.children[i], key)  # @step descend
        if split is not None:
            left, middle, right = split
            node.keys.insert(i, middle)  # @step absorb
            node.children[i:i + 1] = [left, right]
    if len(node.keys) > MAX_KEYS:
        mid = len(node.keys) // 2
        left = Node(node.keys[:mid], node.children[:mid + 1])
        right = Node(node.keys[mid + 1:], node.children[mid + 1:])
        return left, node.keys[mid], right  # @step split
    return None


def run(values):
    root = Node([])
    for value in values:
        split = insert(root, value)
        if split is not None:
            left, middle, right = split
            root = Node([middle], [left, right])  # @step new-root
    out, level = [], [root]
    while level:
        nxt = []
        for node in level:
            out.extend(node.keys)
            out.append(0)
            nxt.extend(node.children)
        level = nxt
    return out
