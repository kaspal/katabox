def validate(inp):
    heights = inp["heights"]
    if not isinstance(heights, list) or not all(type(h) is int for h in heights):
        return "heights is an array of integers"
    if not 1 <= len(heights) <= 10**5:
        return "1 ≤ heights.length ≤ 10⁵"
    if not all(1 <= h <= 10**9 for h in heights):
        return "1 ≤ heights[i] ≤ 10⁹"
    return None
