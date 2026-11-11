"""Two-sum constraints, as the statement writes them."""


def _is_int(x):
    return type(x) is int


def validate(inp):
    nums, target = inp["nums"], inp["target"]
    if not isinstance(nums, list) or not all(_is_int(x) for x in nums) or not _is_int(target):
        return "nums is an array of integers; target is an integer"
    if not 2 <= len(nums) <= 10**5:
        return "2 ≤ nums.length ≤ 10⁵"
    if not all(-(10**9) <= x <= 10**9 for x in nums) or not -(10**9) <= target <= 10**9:
        return "−10⁹ ≤ nums[i], target ≤ 10⁹"
    count = {}
    pairs = 0
    for x in nums:
        pairs += count.get(target - x, 0)
        count[x] = count.get(x, 0) + 1
    if pairs != 1:
        return "Exactly one valid answer exists."
    return None
