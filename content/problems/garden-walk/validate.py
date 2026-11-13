def shape(values):
    """None for a well-formed level-order tree, else the problem."""
    if values and values[0] is None:
        return "the root is null"
    if values and values[-1] is None:
        return "trailing nulls are trimmed"
    slots = 1
    for i, v in enumerate(values):
        if i >= slots:
            return "a node has no parent"
        if v is not None:
            slots += 2
    return None


def height(values):
    level, nodes, i, h = [0] if values else [], values, 1, 0
    while level:
        h += 1
        nxt = []
        for _ in level:
            for _ in range(2):
                if i < len(nodes):
                    if nodes[i] is not None:
                        nxt.append(i)
                    i += 1
        level = nxt
    return h




def tree_problem(root, lo, hi, max_nodes, max_height=1000):
    if not isinstance(root, list) or not all(v is None or type(v) is int for v in root):
        return "root is a level-order list of integers and nulls"
    problem = shape(root)
    if problem:
        return "root is a level-order tree: " + problem
    values = [v for v in root if v is not None]
    if len(values) > max_nodes:
        return "too many nodes"
    if not all(lo <= v <= hi for v in values):
        return "node value out of range"
    if height(root) > max_height:
        return "The tree has at most 1000 levels."
    return None


def validate(inp):
    return tree_problem(inp["root"], -(10**4), 10**4, 10**4)
