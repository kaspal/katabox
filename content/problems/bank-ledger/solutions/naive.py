# Correct but slow: BEGIN copies the whole ledger and COUNT scans it, O(Q · accounts).
import sys


def main() -> None:
    lines = sys.stdin.buffer.read().split(b"\n")
    q = int(lines[0])
    ledger: dict[bytes, int] = {}
    saved: list[dict[bytes, int]] = []
    out: list[str] = []
    for line in lines[1 : 1 + q]:
        parts = line.split()
        cmd = parts[0]
        if cmd == b"SET":
            ledger[parts[1]] = int(parts[2])
        elif cmd == b"ADD":
            ledger[parts[1]] = ledger.get(parts[1], 0) + int(parts[2])
        elif cmd == b"GET":
            v = ledger.get(parts[1])
            out.append("NONE" if v is None else str(v))
        elif cmd == b"COUNT":
            x = int(parts[1])
            c = 0
            for v in ledger.values():
                if v == x:
                    c += 1
            out.append(str(c))
        elif cmd == b"BEGIN":
            saved.append(dict(ledger))
        elif not saved:
            out.append("NO TRANSACTION")
        elif cmd == b"ROLLBACK":
            ledger = saved.pop()
        else:
            saved.pop()
    sys.stdout.write("\n".join(out) + ("\n" if out else ""))


main()
