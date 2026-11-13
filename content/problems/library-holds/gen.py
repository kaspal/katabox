"""Library events."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n events on one popular book: holds, then cancels from members
        # who are not waiting, then returns, as in the max test.
        h, c = size // 2, size * 2 // 5
        r = size - h - c
        ops = ["hold"] * h + ["cancel"] * c + ["return"] * r
        args = [[1, m] for m in range(1, h + 1)] + [[1, m] for m in range(h + 1, h + c + 1)] + [[1]] * r
        return {"ops": ops, "args": args}
    if size == "max":
        # One very popular book with a line of 5·10⁴ members, then events that are
        # slow on a plain list: 4·10⁴ cancels from members who are not waiting (each
        # searches the whole line, 2·10⁹ steps in all) and 10⁴ returns (each shifts
        # the whole line).
        # Seed 1 is the original test; seed 2 is the same shape on another book with other
        # member numbers, so the naive solution's total stays well over its CPU cap.
        n, k, r = 50_000, 40_000, 10_000
        book, base = (1, 0) if seed % 2 else (7, 10**6)
        ops = ["hold"] * n + ["cancel"] * k + ["return"] * r
        args = [[book, base + m] for m in range(1, n + 1)]
        args += [[book, base + m] for m in range(n + 1, n + k + 1)]
        args += [[book]] * r
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

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
