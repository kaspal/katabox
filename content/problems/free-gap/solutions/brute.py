# Obviously correct: mark every busy minute, then walk the day minute by minute.
def minutes(t: str) -> int:
    h, m = t.split(":")
    return int(h) * 60 + int(m)


def main() -> None:
    t = int(input())
    for _ in range(t):
        a, b, k = input().split()
        open_, close = minutes(a), minutes(b)
        busy = [False] * 1440
        for _ in range(int(k)):
            s, e = map(minutes, input().split())
            for m in range(s, e):
                busy[m] = True
        best = None  # (length, start)
        m = open_
        while m < close:
            if busy[m]:
                m += 1
                continue
            start = m
            while m < close and not busy[m]:
                m += 1
            if best is None or m - start > best[0]:
                best = (m - start, start)
        if best is None:
            print("BOOKED")
        else:
            length, start = best
            end = start + length
            print(f"{start // 60:02d}:{start % 60:02d}-{end // 60:02d}:{end % 60:02d}")


main()
