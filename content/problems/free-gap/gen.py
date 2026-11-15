"""Days of busy slots as stdin text (a program problem, ADR 0028)."""
import random


def clock(m):
    return f"{m // 60:02d}:{m % 60:02d}"


def render(days):
    lines = [str(len(days))]
    for open_, close, slots in days:
        lines.append(f"{clock(open_)} {clock(close)} {len(slots)}")
        lines += [f"{clock(s)} {clock(e)}" for s, e in slots]
    return "\n".join(lines) + "\n"


def slots(rng, open_, close, k, longest):
    out = []
    for _ in range(k):
        s = rng.randint(open_, close - 1)
        out.append((s, min(close, s + rng.randint(1, longest))))
    return out


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 20 days of 1000 slots each (the most slots), many overlapping, a few gaps left.
        days = []
        for _ in range(20):
            open_, close = rng.randint(0, 120), rng.randint(1320, 1439)
            days.append((open_, close, slots(rng, open_, close, 1000, 3 if seed % 2 else 40)))
        return render(days)
    if seed % 5 == 0:
        # Edge shapes: no slots, a slot covering the whole day, slots touching end to
        # start, equal gaps (the earliest wins), the whole clock.
        kind = rng.randrange(4)
        if kind == 0:
            return render([(rng.randint(0, 600), 1439, [])])
        if kind == 1:
            return render([(540, 600, [(540, 600)]), (0, 1, [(0, 1)])])
        if kind == 2:
            return render([(480, 720, [(480, 540), (540, 600), (660, 720)])])
        return render([(600, 690, [(630, 660)]), (0, 1439, [(1, 1438)])])
    days = []
    for _ in range(rng.randint(1, 3)):
        open_ = rng.randint(6 * 60, 10 * 60)
        close = open_ + rng.randint(30, 8 * 60)
        days.append((open_, close, slots(rng, open_, close, rng.randint(0, 5), 120)))
    return render(days)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
