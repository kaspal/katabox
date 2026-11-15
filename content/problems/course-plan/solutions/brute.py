# Obviously correct: a course's term is 1 + the latest term among its prerequisites.
# Relax that rule until nothing changes; if terms keep growing past n, there is a loop.
def main() -> None:
    n, m = map(int, input().split())
    rules = [tuple(map(int, input().split())) for _ in range(m)]
    term = [1] * (n + 1)
    for _ in range(n + 1):
        changed = False
        for a, b in rules:
            if term[b] < term[a] + 1:
                term[b] = term[a] + 1
                changed = True
        if not changed:
            break
    if any(t > n for t in term[1:]) or changed:
        print("IMPOSSIBLE")
        return
    last = max(term[1:])
    print(last)
    for t in range(1, last + 1):
        print(" ".join(str(c) for c in range(1, n + 1) if term[c] == t))


main()
