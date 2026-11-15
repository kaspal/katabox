# Obviously correct: BEGIN saves a full copy of the ledger; ROLLBACK goes back to it;
# COMMIT forgets it. COUNT looks at every account.
def main() -> None:
    q = int(input())
    ledger: dict[str, int] = {}
    saved: list[dict[str, int]] = []
    for _ in range(q):
        parts = input().split()
        cmd = parts[0]
        if cmd == "SET":
            ledger[parts[1]] = int(parts[2])
        elif cmd == "ADD":
            ledger[parts[1]] = ledger.get(parts[1], 0) + int(parts[2])
        elif cmd == "GET":
            print(ledger.get(parts[1], "NONE"))
        elif cmd == "COUNT":
            print(sum(1 for v in ledger.values() if v == int(parts[1])))
        elif cmd == "BEGIN":
            saved.append(dict(ledger))
        elif not saved:
            print("NO TRANSACTION")
        elif cmd == "ROLLBACK":
            ledger = saved.pop()
        else:
            saved.pop()


main()
