def plantings(length: int, widths: list[int]) -> int:
    # Obviously correct: build every planting plant by plant.
    def count(left, last):
        if left == 0:
            return 1
        return sum(count(left - w, k) for k, w in enumerate(widths) if k != last and w <= left)

    return count(length, -1) % 1_000_000_007
