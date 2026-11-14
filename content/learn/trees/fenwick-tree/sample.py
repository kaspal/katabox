def add(tree, i, value):
    i += 1
    while i < len(tree):
        tree[i] += value  # @step add
        i += i & -i


def prefix(tree, i):
    i += 1
    total = 0
    while i > 0:
        total += tree[i]  # @step read
        i -= i & -i
    return total


def run(n, ops):
    tree, out = [0] * (n + 1), []
    for op in ops:
        if op[0] == "add":
            add(tree, op[1], op[2])
        else:
            out.append(prefix(tree, op[1]))
    return out
