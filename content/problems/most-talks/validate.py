def validate(inp):
    talks = inp["talks"]
    if not isinstance(talks, list) or not 1 <= len(talks) <= 10**5:
        return "1 ≤ talks.length ≤ 10⁵"
    for t in talks:
        if not isinstance(t, list) or len(t) != 2 or not all(type(v) is int for v in t) or not 0 <= t[0] < t[1] <= 10**6:
            return "0 ≤ talks[i][0] < talks[i][1] ≤ 10⁶"
    return None
