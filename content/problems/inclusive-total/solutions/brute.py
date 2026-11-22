def range_total(nums: list[int], a: int, b: int) -> int:
    # Obviously correct: every position from a to b, both included.
    return sum(nums[i] for i in range(len(nums)) if a <= i <= b)
