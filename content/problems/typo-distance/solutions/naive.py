import sys

sys.setrecursionlimit(10_000)


def typo_distance(typed: str, meant: str) -> int:
    # Correct but exponential: plain recursion on the remaining suffixes.
    def go(i, j):
        if i == len(typed):
            return len(meant) - j
        if j == len(meant):
            return len(typed) - i
        if typed[i] == meant[j]:
            return go(i + 1, j + 1)
        return 1 + min(go(i + 1, j), go(i, j + 1), go(i + 1, j + 1))

    return go(0, 0)
