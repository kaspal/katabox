import sys


def main() -> None:
    tokens = sys.stdin.read().split()
    out: list[str] = []
    for t in tokens[1 : 1 + int(tokens[0])]:
        h, m = int(t[:2]), int(t[3:])
        # In half-degrees, so everything stays whole: the hour hand is at 60·h + m,
        # the minute hand at 12·m.
        gap = abs(60 * (h % 12) - 11 * m)
        gap = min(gap, 720 - gap)
        out.append(f"{gap // 2}.{5 if gap % 2 else 0}")
    sys.stdout.write("\n".join(out) + "\n")


main()
