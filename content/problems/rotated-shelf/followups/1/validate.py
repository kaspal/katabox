"""rotated-shelf--f1 constraints."""


def validate(inp):
    s, q = inp["shelf"], inp["labels"]
    for name, a in (("shelf", s), ("labels", q)):
        if not isinstance(a, list) or not all(type(x) is int for x in a):
            return f"{name} is an array of integers"
    if not 1 <= len(s) <= 10**5 or not 1 <= len(q) <= 10**5:
        return "1 ≤ shelf.length, labels.length ≤ 10⁵"
    if not all(0 <= x <= 10**9 for x in s + q):
        return "0 ≤ shelf[i], labels[k] ≤ 10⁹"
    if len(set(s)) != len(s):
        return "the numbers on the shelf are all different"
    k = s.index(min(s))
    if s[k:] + s[:k] != sorted(s):
        return "shelf is a sorted list with a block moved from the start to the end"
    return None
