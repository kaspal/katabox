import sys


def main() -> None:
    lines = sys.stdin.buffer.read().split(b"\n")
    q = int(lines[0])
    balance: dict[bytes, int] = {}
    holders: dict[int, int] = {}  # balance → how many accounts hold it (for COUNT)
    # One undo log for all open transactions: (name, balance before or None). BEGIN
    # remembers where the log stood; ROLLBACK undoes back to there; COMMIT only forgets
    # the mark, so its changes now belong to the enclosing transaction.
    log: list[tuple[bytes, int | None]] = []
    marks: list[int] = []

    def put(name: bytes, value: int | None) -> None:
        old = balance.get(name)
        if old is not None:
            holders[old] -= 1
        if value is None:
            del balance[name]
        else:
            balance[name] = value
            holders[value] = holders.get(value, 0) + 1

    def change(name: bytes, value: int) -> None:
        if marks:
            log.append((name, balance.get(name)))
        put(name, value)

    out: list[str] = []
    for line in lines[1 : 1 + q]:
        parts = line.split()
        cmd = parts[0]
        if cmd == b"SET":
            change(parts[1], int(parts[2]))
        elif cmd == b"ADD":
            change(parts[1], balance.get(parts[1], 0) + int(parts[2]))
        elif cmd == b"GET":
            v = balance.get(parts[1])
            out.append("NONE" if v is None else str(v))
        elif cmd == b"COUNT":
            out.append(str(holders.get(int(parts[1]), 0)))
        elif cmd == b"BEGIN":
            marks.append(len(log))
        elif not marks:
            out.append("NO TRANSACTION")
        elif cmd == b"ROLLBACK":
            mark = marks.pop()
            while len(log) > mark:
                name, old = log.pop()
                put(name, old)
        else:  # COMMIT
            marks.pop()
            if not marks:
                log.clear()  # nothing can be rolled back any more
    sys.stdout.write("\n".join(out) + ("\n" if out else ""))


main()
