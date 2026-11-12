"""Library events."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # One very popular book with a line of 5·10⁴ members, then events that are
        # slow on a plain list: 4·10⁴ cancels from members who are not waiting (each
        # searches the whole line, 2·10⁹ steps in all) and 10⁴ returns (each shifts
        # the whole line).
        n, k, r = 50_000, 40_000, 10_000
        ops = ["hold"] * n + ["cancel"] * k + ["return"] * r
        args = [[1, m] for m in range(1, n + 1)]
        args += [[1, m] for m in range(n + 1, n + k + 1)]
        args += [[1]] * r
        return {"ops": ops, "args": args}
    books, members = rng.randint(1, 2), rng.randint(1, 4)
    ops, args = [], []
    inline: dict[tuple[int, int], bool] = {}
    for _ in range(rng.randint(1, 12)):
        book = rng.randint(1, books)
        op = rng.choice(["hold", "hold", "cancel", "return"])
        if op == "return":
            ops.append(op)
            args.append([book])
            continue
        member = rng.randint(1, members)
        if op == "hold":
            if inline.get((book, member)):
                op = "cancel"
            inline[(book, member)] = op == "hold"
        else:
            inline[(book, member)] = False
        ops.append(op)
        args.append([book, member])
        # A return can hand the book to anyone, so track the line exactly.
    # Recompute the "in line" state by simulation and drop holds that would repeat.
    clean_ops, clean_args, line = [], [], {}
    for op, a in zip(ops, args):
        q = line.setdefault(a[0], [])
        if op == "hold":
            if a[1] in q:
                continue
            q.append(a[1])
        elif op == "cancel":
            if a[1] in q:
                q.remove(a[1])
        elif q:
            q.pop(0)
        clean_ops.append(op)
        clean_args.append(a)
    if not clean_ops:
        clean_ops, clean_args = ["return"], [[1]]
    return {"ops": clean_ops, "args": clean_args}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
