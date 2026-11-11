def two_sum(nums: list[int], target: int) -> list[int]:
    # Correct but O(n²): must run out of time on the max-size hidden tests.
    n = len(nums)
    for i in range(n):
        for j in range(i + 1, n):
            if nums[i] + nums[j] == target:
                return [i, j]
    return []
