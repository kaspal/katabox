import sys


def seconds(lap: str) -> int:
    m, s = lap.split(":")
    return int(m) * 60 + int(s)


def clock(total: int) -> str:
    return f"{total // 60}:{total % 60:02d}"


def main() -> None:
    tokens = sys.stdin.read().split()
    t = int(tokens[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        name, k = tokens[pos], int(tokens[pos + 1])
        laps = [seconds(x) for x in tokens[pos + 2 : pos + 2 + k]]
        pos += 2 + k
        out.append(f"{name} {clock(sum(laps))} {clock(min(laps))}")
    sys.stdout.write("\n".join(out) + "\n")


main()
