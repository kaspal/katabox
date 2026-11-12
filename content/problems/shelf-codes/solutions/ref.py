def count_below(codes: list[int], queries: list[int]) -> list[int]:
    def first_at_least(x: int) -> int:
        lo, hi = 0, len(codes)
        while lo < hi:
            mid = (lo + hi) // 2
            if codes[mid] < x:
                lo = mid + 1
            else:
                hi = mid
        return lo

    return [first_at_least(x) for x in queries]
