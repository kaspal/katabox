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


def inorder(root):
    """In-order values of a level-order tree, iteratively."""
    left, right, i = {}, {}, 1
    nodes = [j for j, v in enumerate(root) if v is not None]
    for p in nodes:
        if i < len(root):
            if root[i] is not None:
                left[p] = i
            i += 1
        if i < len(root):
            if root[i] is not None:
                right[p] = i
            i += 1
    out, stack, x = [], [], 0 if root else None
    while stack or x is not None:
        while x is not None:
            stack.append(x)
            x = left.get(x)
        x = stack.pop()
        out.append(root[x])
        x = right.get(x)
    return out


def validate(inp):
    root, k = inp["root"], inp["k"]
    problem = tree_problem(root, -(10**9), 10**9, 10**4)
    if problem:
        return problem
    vals = inorder(root)
    if not vals:
        return "1 ≤ number of nodes ≤ 10⁴, and the tree is a valid search tree."
    if any(vals[i] >= vals[i + 1] for i in range(len(vals) - 1)):
        return "1 ≤ number of nodes ≤ 10⁴, and the tree is a valid search tree."
    if type(k) is not int or not 1 <= k <= len(vals):
        return "1 ≤ k ≤ number of nodes"
    return None
