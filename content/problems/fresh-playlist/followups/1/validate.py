"""fresh-playlist--f1 constraints."""


def validate(inp):
    plays, requests = inp["plays"], inp["requests"]
    if not isinstance(plays, list) or not 1 <= len(plays) <= 10**5:
        return "1 ≤ plays.length ≤ 10⁵"
    if not all(type(p) is int and 1 <= p <= 10**9 for p in plays):
        return "1 ≤ plays[i] ≤ 10⁹"
    if not isinstance(requests, list) or not 1 <= len(requests) <= 10**5:
        return "1 ≤ requests.length ≤ 10⁵"
    n = len(plays)
    for r in requests:
        if not (isinstance(r, list) and len(r) == 2 and all(type(v) is int for v in r) and 0 <= r[0] <= r[1] < n):
            return "requests[j] = [a, b] with 0 ≤ a ≤ b < plays.length"
    return None
