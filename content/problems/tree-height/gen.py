"""Random trees (level order with nulls, ADR 0025) for tree-height; height at most 1000."""
import random


def _tree(rng, n, deep):
    # Build node by node: each new node goes under a random open slot (or the deepest one).
    if n == 0:
        return []
    slots = [(0, "L"), (0, "R")]
    children = {0: [None, None]}
    depth = {0: 1}
    for k in range(1, n):
        if deep:
            i = max(range(len(slots)), key=lambda s: depth[slots[s][0]])
        else:
            i = rng.randrange(len(slots))
        parent, side = slots.pop(i)
        if depth[parent] >= 1000:
            slots.append((parent, side))
            continue
        children[parent][0 if side == "L" else 1] = k
        children[k] = [None, None]
        depth[k] = depth[parent] + 1
        slots += [(k, "L"), (k, "R")]
    # Level order with nulls.
    out, queue = [], [0]
    while queue:
        nxt = []
        for node in queue:
            if node is None:
                out.append(None)
                continue
            out.append(rng.randint(-100, 100))
            nxt += children[node]
        queue = nxt if any(x is not None for x in nxt) else []
    while out and out[-1] is None:
        out.pop()
    return out


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n = rng.randint(3, 6)
    elif size == "max":
        n = 10**4
    else:
        n = rng.randint(0, 9)
    return {"root": _tree(rng, n, size == "max" and seed % 2 == 0)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
