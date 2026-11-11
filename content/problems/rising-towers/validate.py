def validate(inp):
    heights = inp["heights"]
    if not isinstance(heights, list) or not all(type(h) is int for h in heights):
        return "heights is an array of integers"
    if not 1 <= len(heights) <= 5 * 10**4:
        return "1 ≤ heights.length ≤ 5·10⁴"
    if not all(1 <= h <= 10**6 for h in heights):
        return "1 ≤ heights[i] ≤ 10⁶"
    return None
