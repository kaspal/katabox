"""Random bracket strings for bracket-checker: many are balanced but for a leftover."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    n = {"example": 6, "max": 10**5}.get(size, rng.randint(1, 10))
    out, stack = [], []
    for _ in range(n):
        if stack and rng.random() < 0.5:
            out.append(stack.pop())
        else:
            k = rng.randrange(3)
            out.append("([{"[k])
            stack.append(")]}"[k])
    s = "".join(out)
    if rng.random() < 0.3 and len(s) > 1:
        i = rng.randrange(len(s))
        s = s[:i] + rng.choice("()[]{}") + s[i + 1 :]
    return {"code": s}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
