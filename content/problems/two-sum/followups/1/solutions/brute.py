def count_pairs(nums: list[int], target: int) -> int:
    # Obviously correct: try every pair of positions.
    return sum(
        1
        for i in range(len(nums))
        for j in range(i + 1, len(nums))
        if nums[i] + nums[j] == target
    )
