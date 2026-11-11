def validate(inp):
    pages, hours = inp["pages"], inp["hours"]
    if not isinstance(pages, list) or not all(type(p) is int for p in pages) or type(hours) is not int:
        return "pages is an array of integers; hours is an integer"
    if not 1 <= len(pages) <= 5 * 10**4:
        return "1 ≤ pages.length ≤ 5·10⁴"
    if not all(1 <= p <= 10**9 for p in pages):
        return "1 ≤ pages[i] ≤ 10⁹"
    if not len(pages) <= hours <= 10**9:
        return "pages.length ≤ hours ≤ 10⁹"
    return None
