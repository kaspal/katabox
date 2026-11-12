"""Ribbons: mirrors with symbols sprinkled in, or near-mirrors with one change."""
import random
import string

WORD = string.ascii_letters + string.digits
SYMBOLS = " .,;:!?-'*#()"


def ribbon(rng, n, mirror):
    half = [rng.choice(WORD) for _ in range(n // 2)]
    core = half + ([rng.choice(WORD)] if n % 2 else []) + half[::-1]
    core = [c.swapcase() if c.isalpha() and rng.random() < 0.3 else c for c in core]
    if not mirror and len(core) >= 2:
        k = rng.randrange(len(core) // 2)
        options = [c for c in WORD if c.lower() != core[-1 - k].lower()]
        core[k] = rng.choice(options)
    out = []
    for c in core:
        while rng.random() < 0.3:
            out.append(rng.choice(SYMBOLS))
        out.append(c)
    return "".join(out)


def generate(seed, size):
    rng = random.Random(seed)
    mirror = seed % 2 == 1
    if size == "max":
        # About 86 000 characters (60 000 letters and digits plus symbols).
        text = ribbon(rng, 60_000, mirror)
        assert len(text) <= 100_000
    else:
        text = ribbon(rng, rng.randint(0, 7), mirror) or rng.choice(SYMBOLS)
    return {"text": text}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
