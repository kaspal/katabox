"""Course prerequisite lists as stdin text (a program problem, ADR 0028)."""
import random


def render(n, edges):
    lines = [f"{n} {len(edges)}"] + [f"{a} {b}" for a, b in edges]
    return "\n".join(lines) + "\n"


def chain(rng, n, extra):
    """A hidden order of the n courses (a chain: n terms) plus `extra` prerequisites that
    agree with it, with the course numbers shuffled."""
    label = list(range(1, n + 1))
    rng.shuffle(label)
    edges = [(label[i], label[i + 1]) for i in range(n - 1)]
    for _ in range(extra):
        i, j = sorted(rng.sample(range(n), 2))
        edges.append((label[i], label[j]))
    rng.shuffle(edges)
    return edges


def sized(rng, n):
    """Complexity profile input (ADR 0024): a chain of n courses and n // 2 more rules."""
    return render(n, chain(rng, n, n // 2))


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 3·10⁴ courses in a hidden chain (3·10⁴ terms of one course each) plus 10⁴
            # more rules: rescanning every course each term is about 10⁹ steps or more.
            n = 30_000
            return render(n, chain(rng, n, 10_000))
        # 10⁴ courses: a chain of 9 990, then a loop among the last ten (IMPOSSIBLE only
        # after the chain is done), with repeated rules.
        n = 10_000
        edges = chain(rng, n - 10, 2_000)
        loop = list(range(n - 9, n + 1))
        edges += [(loop[i], loop[(i + 1) % 10]) for i in range(10)]
        edges.append((rng.randint(1, n - 10), loop[0]))
        edges += rng.sample(edges, 500)
        rng.shuffle(edges)
        return render(n, edges)
    if seed % 5 == 0:
        # Edge shapes: one course, no rules, a rule repeated, a two-course loop, a
        # course that requires itself through a longer loop.
        kind = rng.randrange(4)
        if kind == 0:
            return render(rng.randint(1, 5), [])
        if kind == 1:
            return render(3, [(1, 3)] * rng.randint(2, 4) + [(2, 3)])
        if kind == 2:
            return render(rng.randint(2, 5), [(1, 2), (2, 1)])
        return render(5, [(1, 2), (2, 3), (3, 1), (4, 5)])
    n = rng.randint(1, 8)
    edges = []
    order = list(range(1, n + 1))
    rng.shuffle(order)
    for _ in range(rng.randint(0, 2 * n)):
        if n < 2:
            break
        i, j = sorted(rng.sample(range(n), 2))
        edges.append((order[i], order[j]))
    if n >= 2 and rng.random() < 0.25:
        i, j = sorted(rng.sample(range(n), 2))
        edges.append((order[j], order[i]))  # against the order: maybe a loop
    return render(n, edges)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
