def validate(inp):
    t = inp["text"]
    if not isinstance(t, str):
        return "text is a string"
    if len(t) > 10**5:
        return "0 ≤ text.length ≤ 10⁵"
    if not all(32 <= ord(c) <= 126 for c in t):
        return "text has printable ASCII characters only"
    return None
