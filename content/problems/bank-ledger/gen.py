"""Ledger commands as stdin text (a program problem, ADR 0028)."""
import random
import string


def names(rng, count, low, high):
    out = set()
    while len(out) < count:
        out.add("".join(rng.choice(string.ascii_lowercase) for _ in range(rng.randint(low, high))))
    return sorted(out)


def session(rng, accounts, q, max_depth):
    """Open every account, then q random commands: many COUNTs and BEGINs, transactions
    nested at most max_depth deep."""
    cmds = [f"SET {a} {rng.randint(0, 99)}" for a in accounts]
    depth = 0
    while len(cmds) < q:
        r = rng.random()
        if r < 0.35:
            cmds.append(f"COUNT {rng.randint(-50, 150)}")
        elif r < 0.47 and depth < max_depth:
            cmds.append("BEGIN")
            depth += 1
        elif r < 0.53:
            cmds.append("ROLLBACK")
            depth = max(0, depth - 1)
        elif r < 0.59:
            cmds.append("COMMIT")
            depth = max(0, depth - 1)
        elif r < 0.77:
            cmds.append(f"SET {rng.choice(accounts)} {rng.randint(0, 99)}")
        elif r < 0.9:
            cmds.append(f"ADD {rng.choice(accounts)} {rng.randint(-50, 50)}")
        else:
            cmds.append(f"GET {rng.choice(accounts)}")
    return cmds


def render(cmds):
    return "\n".join([str(len(cmds))] + cmds) + "\n"


def sized(rng, n):
    """Complexity profile input (ADR 0024): n // 4 accounts, n commands in all."""
    return render(session(rng, names(rng, max(1, n // 4), 3, 4), n, 30))


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 2.5·10⁴ accounts and 8·10⁴ commands: copying every account at each BEGIN and
            # scanning them all for each COUNT is about 10⁹ steps.
            return render(session(rng, names(rng, 25_000, 3, 4), 80_000, 40))
        # Deep nesting: 3000 BEGINs in a row, changes at every level, then unwinding with
        # a mix of COMMIT and ROLLBACK; large amounts.
        accounts = names(rng, 50, 1, 10)
        cmds = []
        for _ in range(3000):
            cmds.append("BEGIN")
            a = rng.choice(accounts)
            cmds.append(f"SET {a} {rng.randint(0, 1_000_000_000)}" if rng.random() < 0.5 else f"ADD {a} {rng.randint(-1_000_000_000, 1_000_000_000)}")
            if rng.random() < 0.2:
                cmds.append(f"GET {rng.choice(accounts)}")
        for _ in range(3001):
            cmds.append(rng.choice(["COMMIT", "ROLLBACK"]))
            cmds.append(f"GET {rng.choice(accounts)}")
        cmds.append("COUNT 0")
        return render(cmds)
    if seed % 5 == 0:
        # Edge shapes: COMMIT and ROLLBACK with nothing open, a rollback that removes an
        # account, ADD creating an account, negative balances, COUNT of a missing value.
        kind = rng.randrange(3)
        if kind == 0:
            return render(["COMMIT", "ROLLBACK", "GET x", "COUNT 0"])
        if kind == 1:
            return render(["BEGIN", "ADD x -7", "COUNT -7", "BEGIN", "SET y 0", "ROLLBACK", "GET y", "COMMIT", "GET x", "COMMIT"])
        return render(["SET a 1000000000", "ADD a 1000000000", "BEGIN", "ADD a -1000000000", "COUNT 1000000000", "ROLLBACK", "COUNT 2000000000", "GET a"])
    return render(session(rng, names(rng, rng.randint(1, 3), 1, 2), rng.randint(4, 14), 3))


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
