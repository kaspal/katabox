# Obviously correct: whole-list builtins, pairs of neighbours, and groups of equal readings.
import sys
from itertools import groupby


def main() -> None:
    lines = sys.stdin.read().split("\n")
    n = int(lines[0])
    readings = [int(line) for line in lines[1 : 1 + n]]
    total = sum(readings)
    rises = sum(1 for a, b in zip(readings, readings[1:]) if b > a)
    longest = max(len(list(group)) for _, group in groupby(readings))
    print(total, rises, longest)


main()
