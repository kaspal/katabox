def build(tree, a, node, lo, hi):
    if lo == hi:
        tree[node] = a[lo]  # @step leaf
        return
    mid = (lo + hi) // 2
    build(tree, a, 2 * node, lo, mid)
    build(tree, a, 2 * node + 1, mid + 1, hi)
    tree[node] = tree[2 * node] + tree[2 * node + 1]  # @step merge


def query(tree, node, lo, hi, left, right):
    if right < lo or hi < left:
        return 0  # @step outside
    if left <= lo and hi <= right:
        return tree[node]  # @step inside
    mid = (lo + hi) // 2
    return (query(tree, 2 * node, lo, mid, left, right)
            + query(tree, 2 * node + 1, mid + 1, hi, left, right))


def update(tree, node, lo, hi, i, value):
    if lo == hi:
        tree[node] = value  # @step set
        return
    mid = (lo + hi) // 2
    if i <= mid:
        update(tree, 2 * node, lo, mid, i, value)
    else:
        update(tree, 2 * node + 1, mid + 1, hi, i, value)
    tree[node] = tree[2 * node] + tree[2 * node + 1]  # @step recompute


def run(a, ops):
    n = len(a)
    tree = [0] * (4 * n)
    build(tree, a, 1, 0, n - 1)
    out = []
    for op in ops:
        if op[0] == "sum":
            out.append(query(tree, 1, 0, n - 1, op[1], op[2]))
        else:
            update(tree, 1, 0, n - 1, op[1], op[2])
    return out
