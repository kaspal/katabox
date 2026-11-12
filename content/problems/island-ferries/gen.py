"""Archipelagos: random lines, or a long chain listed from its far end."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n islands, the first half a chain listed from the far end.
        chain = max(2, size // 2)
        return {"n": size, "links": [[i, i + 1] for i in range(chain - 2, -1, -1)]}
    if size == "max":
        # A chain 0-1-2-...-49 999 whose lines are listed from the far end, plus 5·10⁴
        # islands without ferries. Relabelling until stable needs one pass per island
        # of the chain: about 2.5·10⁹ steps.
        chain = 50_000
        n = 100_000
        links = [[i, i + 1] for i in range(chain - 2, -1, -1)]
    else:
        n = rng.randint(1, 8)
        links = []
        if n >= 2:
            for _ in range(rng.randint(0, 8)):
                a, b = rng.sample(range(n), 2)
                links.append([a, b])
    return {"n": n, "links": links}


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
