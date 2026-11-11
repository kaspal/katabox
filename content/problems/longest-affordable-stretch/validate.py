def validate(inp):
    costs, budget = inp["costs"], inp["budget"]
    if not isinstance(costs, list) or not all(type(c) is int for c in costs) or type(budget) is not int:
        return "costs is an array of integers; budget is an integer"
    if not 1 <= len(costs) <= 5 * 10**4:
        return "1 ≤ costs.length ≤ 5·10⁴"
    if not all(0 <= c <= 10**4 for c in costs):
        return "0 ≤ costs[i] ≤ 10⁴"
    if not 0 <= budget <= 10**9:
        return "0 ≤ budget ≤ 10⁹"
    return None
