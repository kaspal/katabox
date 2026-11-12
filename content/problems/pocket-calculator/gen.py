"""Calculator expressions."""
import random

LIMIT = 2**53


def build(rng, depth, budget):
    """(text, value) of a random expression whose every intermediate stays small."""
    for _ in range(50):
        kind = rng.random()
        if depth <= 0 or budget <= 1 or kind < 0.3:
            v = rng.randint(0, 99)
            text, value = str(v), v
        elif kind < 0.4:
            t, v = build(rng, depth - 1, budget - 1)
            text, value = "-" + ("(" + t + ")" if rng.random() < 0.7 else t if t[0] not in "-" else "(" + t + ")"), -v
        elif kind < 0.55:
            t, v = build(rng, depth - 1, budget - 1)
            text, value = "(" + t + ")", v
        else:
            op = rng.choice("+-*")
            a, va = build(rng, depth - 1, budget // 2)
            b, vb = build(rng, depth - 1, budget // 2)
            if op == "*":
                a, b = "(" + a + ")", "(" + b + ")"
                value = va * vb
            elif op == "+":
                b = "(" + b + ")"
                value = va + vb
            else:
                b = "(" + b + ")"
                value = va - vb
            sp = rng.choice(["", " "])
            text = a + sp + op + sp + b
        if abs(value) < LIMIT:
            return text, value
    return "1", 1


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # A long sum of products: about 10⁴ characters.
        parts = []
        length = 0
        while length < 9_900:
            p = " * ".join(str(rng.randint(1, 99)) for _ in range(rng.randint(1, 3)))
            parts.append(p)
            length += len(p) + 3
        return {"expression": (" + " if rng.random() < 0.5 else " - ").join(parts)[:10_000].rstrip(" +-*")}
    return {"expression": build(rng, rng.randint(1, 4), 12)[0]}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
