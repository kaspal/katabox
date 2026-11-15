"""Test cases of numbers and quotas as stdin text (a program problem, ADR 0028)."""
import random


def render(cases):
    lines = [str(len(cases))]
    for k, values in cases:
        lines.append(f"{len(values)} {k}")
        lines.append(" ".join(map(str, values)))
    return "\n".join(lines) + "\n"


def sized(rng, n):
    """Complexity profile input (ADR 0024): one case of n values, like "max"."""
    return render([(rng.randint(-200, 200), [rng.randint(-999, 999) for _ in range(n)])])


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # One case of 10⁵ values: about 5·10⁹ pairs to try one by one. The values are
            # short (three digits) so the input stays small; the quota keeps the count near
            # half of the pairs.
            cases = [(rng.randint(-200, 200), [rng.randint(-999, 999) for _ in range(100_000)])]
        else:
            # 2000 cases of 10 values: many small cases, values up to ±10⁹.
            cases = []
            for _ in range(2000):
                values = [rng.randint(-1_000_000_000, 1_000_000_000) for _ in range(10)]
                cases.append((rng.randint(-2_000_000_000, 2_000_000_000), values))
    elif seed % 5 == 0:
        # Edge shapes: one value, equal values, the extreme values and quotas.
        cases = []
        for _ in range(rng.randint(1, 4)):
            kind = rng.randrange(3)
            if kind == 0:
                cases.append((rng.randint(-5, 5), [rng.randint(-5, 5)]))
            elif kind == 1:
                v = rng.choice([-1_000_000_000, 0, 1_000_000_000])
                k = max(-2_000_000_000, min(2_000_000_000, rng.choice([2 * v - 1, 2 * v, 2 * v + 1])))
                cases.append((k, [v] * rng.randint(2, 6)))
            else:
                values = [rng.choice([-1_000_000_000, 1_000_000_000]) for _ in range(rng.randint(2, 6))]
                cases.append((rng.choice([-2_000_000_000, 0, 2_000_000_000]), values))
    else:
        cases = []
        for _ in range(rng.randint(1, 4)):
            values = [rng.randint(-20, 20) for _ in range(rng.randint(1, 8))]
            cases.append((rng.randint(-25, 25), values))
    return render(cases)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
