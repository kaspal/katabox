def apply(tree, lazy, node, lo, hi, value):
    tree[node] += value * (hi - lo + 1)
    lazy[node] += value


def push(tree, lazy, node, lo, hi):
    if lazy[node] != 0:
        mid = (lo + hi) // 2
        apply(tree, lazy, 2 * node, lo, mid, lazy[node])  # @step push-down
        apply(tree, lazy, 2 * node + 1, mid + 1, hi, lazy[node])
        lazy[node] = 0


def add(tree, lazy, node, lo, hi, left, right, value):
    if right < lo or hi < left:
        return
    if left <= lo and hi <= right:
        apply(tree, lazy, node, lo, hi, value)  # @step tag
        return
    push(tree, lazy, node, lo, hi)
    mid = (lo + hi) // 2
    add(tree, lazy, 2 * node, lo, mid, left, right, value)
    add(tree, lazy, 2 * node + 1, mid + 1, hi, left, right, value)
    tree[node] = tree[2 * node] + tree[2 * node + 1]  # @step recompute


def total(tree, lazy, node, lo, hi, left, right):
    if right < lo or hi < left:
        return 0
    if left <= lo and hi <= right:
        return tree[node]  # @step inside
    push(tree, lazy, node, lo, hi)
    mid = (lo + hi) // 2
    return (total(tree, lazy, 2 * node, lo, mid, left, right)
            + total(tree, lazy, 2 * node + 1, mid + 1, hi, left, right))


def run(n, ops):
    tree, lazy = [0] * (4 * n), [0] * (4 * n)
    out = []
    for op in ops:
        if op[0] == "add":
            add(tree, lazy, 1, 0, n - 1, op[1], op[2], op[3])
        else:
            out.append(total(tree, lazy, 1, 0, n - 1, op[1], op[2]))
    return out
