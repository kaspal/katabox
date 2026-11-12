"""Bracelet engravings."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 2·10⁴ codes of length 12, nearly all different bracelets: every new code is
        # compared with almost every earlier one. Three letters keep the file small and
        # still allow 3¹² ≈ 531 000 different codes.
        n = 20_000
        alphabet = "xyz"
        codes = ["".join(rng.choice(alphabet) for _ in range(12)) for _ in range(n)]
    else:
        alphabet = rng.choice(["ab", "abc", "lamp"])
        codes = []
        for _ in range(rng.randint(1, 7)):
            if codes and rng.random() < 0.4:
                c = rng.choice(codes)
                i = rng.randrange(len(c))
                codes.append(c[i:] + c[:i])
            else:
                codes.append("".join(rng.choice(alphabet) for _ in range(rng.randint(1, 5))))
    return {"codes": codes}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
