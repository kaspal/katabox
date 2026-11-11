"""Random bracket lines: about half are properly nested."""
import random

OPEN, CLOSE = "([{", ")]}"


def _nested(rng, pairs, letters):
    out, stack = [], []
    remaining = pairs
    while remaining or stack:
        if rng.random() < letters:
            out.append(rng.choice("abcxyz"))
        if remaining and (not stack or rng.random() < 0.55):
            k = rng.randrange(3)
            out.append(OPEN[k])
            stack.append(k)
            remaining -= 1
        else:
            out.append(CLOSE[stack.pop()])
    return "".join(out)


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Deep nesting is what makes pair deletion slow.
        depth = 50_000
        kinds = [rng.randrange(3) for _ in range(depth)]
        text = "".join(OPEN[k] for k in kinds) + "".join(CLOSE[k] for k in reversed(kinds))
        if seed % 2 == 0:
            # Break it at the very end: the last bracket closes the wrong kind.
            last = CLOSE[(kinds[0] + 1) % 3]
            text = text[:-1] + last
        return {"text": text}
    text = _nested(rng, rng.randint(0, 5), 0.2)
    if rng.random() < 0.5 and text:
        # Mutate one character: usually breaks the nesting.
        i = rng.randrange(len(text))
        text = text[:i] + rng.choice(OPEN + CLOSE + "q") + text[i + 1:]
    if not text:
        text = rng.choice(["a", "()", ")"])
    return {"text": text}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
