"""Random trees with distinct values, given as preorder and inorder."""
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




def orders(root):
    """Preorder and inorder of a level-order tree."""
    left, right, i = {}, {}, 1
    for p in [j for j, v in enumerate(root) if v is not None]:
        for side in (left, right):
            if i < len(root):
                if root[i] is not None:
                    side[p] = i
                i += 1
    pre, ino = [], []
    if root:
        stack = [0]
        while stack:
            x = stack.pop()
            pre.append(root[x])
            for c in (right.get(x), left.get(x)):
                if c is not None:
                    stack.append(c)
        stack, x = [], 0
        while stack or x is not None:
            while x is not None:
                stack.append(x)
                x = left.get(x)
            x = stack.pop()
            ino.append(root[x])
            x = right.get(x)
    return pre, ino


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 10_000
        root = random_tree(rng, n, 0, 0, 1000, 1000 if seed % 2 else 0)
    else:
        n = rng.randint(0, 8)
        root = random_tree(rng, n, 0, 0, 1000)
    span = 10**6 if size == "max" else 30
    vals = iter(rng.sample(range(-span, span + 1), n))
    root = [None if v is None else next(vals) for v in root]
    pre, ino = orders(root)
    return {"preorder": pre, "inorder": ino}


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
