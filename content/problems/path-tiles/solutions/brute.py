def tilings(n: int) -> int:
    # Obviously correct: build every sequence of pieces.
    def count(left):
        if left == 0:
            return 1
        return sum(count(left - p) for p in (1, 2, 3) if p <= left)

    return count(n) % 1_000_000_007
