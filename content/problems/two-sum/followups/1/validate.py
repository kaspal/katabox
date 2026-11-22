"""two-sum--f1 constraints, as the statement writes them."""


def _is_int(x):
    return type(x) is int


def validate(inp):
    nums, target = inp["nums"], inp["target"]
    if not isinstance(nums, list) or not all(_is_int(x) for x in nums) or not _is_int(target):
        return "nums is an array of integers; target is an integer"
    if not 1 <= len(nums) <= 10**5:
        return "1 ≤ nums.length ≤ 10⁵"
    if not all(-(10**9) <= x <= 10**9 for x in nums):
        return "−10⁹ ≤ nums[i] ≤ 10⁹"
    if not -2 * 10**9 <= target <= 2 * 10**9:
        return "−2·10⁹ ≤ target ≤ 2·10⁹"
    return None
