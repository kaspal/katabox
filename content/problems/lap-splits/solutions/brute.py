# Obviously correct: line by line, laps as (minutes, seconds), carrying seconds into minutes.
def main() -> None:
    t = int(input())
    for _ in range(t):
        parts = input().split()
        who, k, laps = parts[0], int(parts[1]), parts[2:]
        assert len(laps) == k
        pairs = [tuple(int(x) for x in p.split(":")) for p in laps]
        minutes = sum(m for m, _ in pairs)
        seconds = sum(s for _, s in pairs)
        while seconds >= 60:
            seconds -= 60
            minutes += 1
        best = min(pairs)
        print(who, f"{minutes}:{seconds:02d}", f"{best[0]}:{best[1]:02d}")


main()
