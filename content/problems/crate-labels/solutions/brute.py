# Obviously correct: list every (name, quantity) pair, then add up each distinct name.
def main() -> None:
    n = int(input())
    pairs = []
    for _ in range(n):
        for entry in input().split(";"):
            name, qty = entry.split(":")
            pairs.append((name, int(qty)))
    for name in sorted({name for name, _ in pairs}):
        print(name, sum(q for nm, q in pairs if nm == name))


main()
