"""most-talks--f1 constraints."""


def validate(inp):
    talks = inp["talks"]
    if not isinstance(talks, list) or not 1 <= len(talks) <= 10**5:
        return "1 ≤ talks.length ≤ 10⁵"
    for t in talks:
        if not (isinstance(t, list) and len(t) == 3 and all(type(x) is int for x in t)):
            return "talks[i] = [start, end, score]"
        s, e, score = t
        if not (0 <= s < e <= 10**6 and 1 <= score <= 10**4):
            return "0 ≤ start < end ≤ 10⁶ and 1 ≤ score ≤ 10⁴"
    return None
