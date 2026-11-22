def count_pairs(nums: list[int], target: int) -> int:
    seen: dict[int, int] = {}
    pairs = 0
    for n in nums:
        pairs += seen.get(target - n, 0)
        seen[n] = seen.get(n, 0) + 1
    return pairs
