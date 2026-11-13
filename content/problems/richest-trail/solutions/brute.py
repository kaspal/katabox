# Provided by KataBox: the judge builds the input trees with this class.
class TreeNode:
    def __init__(self, val: int = 0, left: TreeNode | None = None, right: TreeNode | None = None):
        self.val = val
        self.left = left
        self.right = right


def richest_trail(root: TreeNode | None) -> int:
    # Obviously correct: build the graph, then sum the unique path between every pair.
    parent, nodes, todo = {}, [], [root]
    while todo:
        node = todo.pop()
        nodes.append(node)
        for child in (node.left, node.right):
            if child is not None:
                parent[id(child)] = node
                todo.append(child)

    def path_up(node):
        out = [node]
        while id(out[-1]) in parent:
            out.append(parent[id(out[-1])])
        return out

    best = None
    for a in nodes:
        up_a = path_up(a)
        ids_a = {id(x): i for i, x in enumerate(up_a)}
        for b in nodes:
            up_b = path_up(b)
            j = next(j for j, x in enumerate(up_b) if id(x) in ids_a)
            i = ids_a[id(up_b[j])]
            total = sum(x.val for x in up_a[: i + 1]) + sum(x.val for x in up_b[:j])
            best = total if best is None else max(best, total)
    return best
