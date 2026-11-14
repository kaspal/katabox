"""Radio messages as stdin text (a program problem, ADR 0028)."""
import random

EDGES = ["0", "10", "20", "30", "100", "101", "110", "1010", "27", "26", "2626", "01", "230", "1111111111", "2222222222", "9"]


def message(rng, n, zeros=0.12):
    """n digits rich in 1s and 2s (many readings); a 0 only right after a 1 or a 2."""
    digits = []
    prev = ""
    for _ in range(n):
        if prev in ("1", "2") and rng.random() < zeros:
            d = "0"
        else:
            r = rng.random()
            d = "1" if r < 0.4 else "2" if r < 0.75 else rng.choice("3456789")
        digits.append(d)
        prev = d
    return "".join(digits)


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 2·10⁵ digits in all. The first message is long and readable in
        # astronomically many ways, so listing the readings one by one never ends.
        shape = seed % 3
        if shape == 0:
            lengths = [100_000, 100_000]
        elif shape == 1:
            lengths = [10_000] * 20
        else:
            lengths = [100_000] + [100] * 999  # T = 1000, the most messages allowed
        msgs = [message(rng, n) for n in lengths]
        # A few later messages get an unreadable spot (a "00" or a "30") somewhere.
        for j in range(1, len(msgs)):
            if rng.random() < 0.2:
                at = rng.randrange(len(msgs[j]) - 1)
                bad = rng.choice(["00", "30"])
                msgs[j] = msgs[j][:at] + bad + msgs[j][at + 2 :]
    elif seed % 5 == 0:
        msgs = [rng.choice(EDGES) for _ in range(rng.randint(1, 4))]
    else:
        msgs = []
        for _ in range(rng.randint(1, 4)):
            n = rng.randint(1, 10)
            if rng.random() < 0.6:
                msgs.append(message(rng, n, zeros=0.25))
            else:
                msgs.append("".join(rng.choice("0012223456789") for _ in range(n)))
    lines = [str(len(msgs))] + ["#" + m for m in msgs]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
