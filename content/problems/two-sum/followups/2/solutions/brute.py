def closest_sum(nums: list[int], target: int) -> int:
    # Obviously correct: every pair, closest first, then the smaller sum.
    sums = [nums[i] + nums[j] for i in range(len(nums)) for j in range(i + 1, len(nums))]
    return min(sums, key=lambda s: (abs(s - target), s))
