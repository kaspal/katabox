def validate(inp):
    sales, k = inp["sales"], inp["k"]
    if not isinstance(sales, list) or not all(type(x) is int for x in sales) or type(k) is not int:
        return "sales is an array of integers; k is an integer"
    if not 1 <= k <= len(sales) <= 2 * 10**5:
        return "1 ≤ k ≤ sales.length ≤ 2·10⁵"
    if not all(0 <= x <= 10**4 for x in sales):
        return "0 ≤ sales[i] ≤ 10⁴"
    return None
