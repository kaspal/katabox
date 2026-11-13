# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def brightest_street(root: TreeNode | None) -> int:
    # Obviously correct (small trees): try every set of lit posts.
    nodes, parent, todo = [], {}, [root] if root is not None else []
    while todo:
        node = todo.pop()
        nodes.append(node)
        for child in (node.left, node.right):
            if child is not None:
                parent[id(child)] = node
                todo.append(child)
    best = 0
    for mask in range(1 << len(nodes)):
        lit = {id(nodes[i]) for i in range(len(nodes)) if mask >> i & 1}
        if any(id(x) in lit and id(parent[id(x)]) in lit for x in nodes if id(x) in parent):
            continue
        best = max(best, sum(x.val for x in nodes if id(x) in lit))
    return best
