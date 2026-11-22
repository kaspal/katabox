def range_total(nums: list[int], a: int, b: int) -> int:
    total = 0
    for i in range(a, b):
        total += nums[i]
    return total
