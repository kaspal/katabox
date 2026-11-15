import sys


def minutes(t: bytes) -> int:
    return int(t[:2]) * 60 + int(t[3:])


def clock(m: int) -> str:
    return f"{m // 60:02d}:{m % 60:02d}"


def main() -> None:
    data = sys.stdin.buffer.read().split()
    t = int(data[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        open_, close, k = minutes(data[pos]), minutes(data[pos + 1]), int(data[pos + 2])
        slots = sorted((minutes(data[pos + 3 + 2 * i]), minutes(data[pos + 4 + 2 * i])) for i in range(k))
        pos += 3 + 2 * k
        # Sweep the slots by start time; `free_from` is where the free time begins. A gap
        # is only replaced by a strictly longer one, so the earliest longest gap wins.
        best_len, best_start = 0, 0
        free_from = open_
        for s, e in slots:
            if s - free_from > best_len:
                best_len, best_start = s - free_from, free_from
            free_from = max(free_from, e)
        if close - free_from > best_len:
            best_len, best_start = close - free_from, free_from
        out.append(f"{clock(best_start)}-{clock(best_start + best_len)}" if best_len else "BOOKED")
    sys.stdout.write("\n".join(out) + "\n")


main()
