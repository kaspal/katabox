"""two-sum--f2 constraints."""


def validate(inp):
    nums, target = inp["nums"], inp["target"]
    if not isinstance(nums, list) or not all(type(x) is int for x in nums) or type(target) is not int:
        return "nums is an array of integers; target is an integer"
    if not 2 <= len(nums) <= 10**5:
        return "2 ≤ nums.length ≤ 10⁵"
    if not all(-(10**9) <= x <= 10**9 for x in nums):
        return "−10⁹ ≤ nums[i] ≤ 10⁹"
    if not -2 * 10**9 <= target <= 2 * 10**9:
        return "−2·10⁹ ≤ target ≤ 2·10⁹"
    return None
