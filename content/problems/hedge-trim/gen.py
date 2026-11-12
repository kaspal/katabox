"""Random binary trees of vigor scores in level order (height at most 1000)."""
import random


def level_order(nodes, left, right):
    """Level order of the tree rooted at node 0, trailing nulls trimmed."""
    if not nodes:
        return []
    out, queue, i = [], [0], 0
    while i < len(queue):
        x = queue[i]
        i += 1
        if x is None:
            out.append(None)
            continue
        out.append(nodes[x])
        queue.append(left[x])
        queue.append(right[x])
    while out and out[-1] is None:
        out.pop()
    return out


def random_tree(rng, n, lo, hi, max_height, chain=0):
    """n nodes; the first `chain` nodes form a path (deep), the rest hang randomly."""
    if n == 0:
        return []
    vals = [rng.randint(lo, hi) for _ in range(n)]
    left, right, depth = [None] * n, [None] * n, [1] * n
    free = [0]
    for x in range(1, n):
        if x < chain:
            parent = x - 1
            side = rng.choice("lr")
            if side == "l":
                left[parent] = x
            else:
                right[parent] = x
        else:
            while True:
                parent = rng.choice(free)
                options = [s for s, c in (("l", left[parent]), ("r", right[parent])) if c is None]
                if options and depth[parent] < max_height:
                    break
                free.remove(parent)
            if rng.choice(options) == "l":
                left[parent] = x
            else:
                right[parent] = x
        depth[x] = depth[parent] + 1
        free.append(x)
    return level_order(vals, left, right)


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        chain = 1000 if seed % 2 else 0
        # Mostly healthy with sickly patches, so cuts happen at every depth.
        return {"root": random_tree(rng, 10_000, -40, 60, 1000, chain)}
    return {"root": random_tree(rng, rng.randint(0, 9), -6, 6, 1000)}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
