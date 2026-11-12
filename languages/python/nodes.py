# ---- Node types (ADR 0025): JSON <-> ListNode / TreeNode ------------------------------
# A list travels as its values in order; a tree as its level order with None for a
# missing child (children of a missing node are not listed, trailing Nones trimmed).
# Nodes are built with the solution's own classes (its starter code defines them), without
# running their constructors, and read back by duck typing; everything is iterative, so
# deep structures are fine.

NODE_LIMIT = 1_000_000


class NodeError(Exception):
    """A returned structure the judge refuses (a cycle, or too many nodes)."""

    def __init__(self, code, message):
        super().__init__(message)
        self.code = code


class _ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class _TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def node_classes(module):
    """The solution's ListNode and TreeNode classes, or the judge's own."""

    def pick(name, fallback):
        cls = getattr(module, name, None)
        return cls if isinstance(cls, type) else fallback

    return {"list": pick("ListNode", _ListNode), "tree": pick("TreeNode", _TreeNode)}


def _node(cls, **fields):
    node = cls.__new__(cls)
    for key, value in fields.items():
        setattr(node, key, value)
    return node


def list_from_json(cls, values):
    head = None
    for v in reversed(values):
        head = _node(cls, val=v, next=head)
    return head


def tree_from_json(cls, values):
    if not values:
        return None
    nodes = [None if v is None else _node(cls, val=v, left=None, right=None) for v in values]
    i = 1
    for parent in nodes:
        if i >= len(nodes):
            break
        if parent is None:
            continue
        parent.left = nodes[i]
        i += 1
        if i < len(nodes):
            parent.right = nodes[i]
            i += 1
    return nodes[0]


def list_to_json(node):
    out, seen = [], set()
    while node is not None:
        if id(node) in seen:
            raise NodeError("node_cycle", "the returned list links back to a node it already contains (a cycle)")
        seen.add(id(node))
        if len(out) >= NODE_LIMIT:
            raise NodeError("return_not_serializable", "the returned list has more than %d nodes" % NODE_LIMIT)
        out.append(node.val)
        node = node.next
    return out


def tree_to_json(root):
    if root is None:
        return []
    out, seen, queue, i = [], set(), [root], 0
    while i < len(queue):
        node = queue[i]
        i += 1
        if node is None:
            out.append(None)
            continue
        if id(node) in seen:
            raise NodeError("node_cycle", "the returned tree reaches the same node twice (a cycle or a shared node)")
        seen.add(id(node))
        if len(seen) > NODE_LIMIT:
            raise NodeError("return_not_serializable", "the returned tree has more than %d nodes" % NODE_LIMIT)
        out.append(node.val)
        queue.append(node.left)
        queue.append(node.right)
    while out and out[-1] is None:
        out.pop()
    return out


def from_json(shape, depth, value, classes):
    """A JSON value as the solution receives it (`shape` is "list", "tree" or "plain")."""
    if shape == "plain":
        return value
    if depth:
        return [from_json(shape, depth - 1, v, classes) for v in value]
    if shape == "list":
        return list_from_json(classes["list"], value)
    return tree_from_json(classes["tree"], value)


def to_json(shape, depth, value):
    """A returned value as JSON (raises NodeError, or AttributeError for a non-node)."""
    if shape == "plain":
        return value
    if depth:
        if not isinstance(value, (list, tuple)):
            raise TypeError("expected a list, got " + type(value).__name__)
        return [to_json(shape, depth - 1, v) for v in value]
    return list_to_json(value) if shape == "list" else tree_to_json(value)
