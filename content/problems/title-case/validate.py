import string


def validate(inp):
    t = inp["text"]
    if not isinstance(t, str):
        return "text is a string"
    if len(t) > 10**4:
        return "0 ≤ text.length ≤ 10⁴"
    if not all(c in string.ascii_letters + string.digits + " " for c in t):
        return "text has English letters, digits and spaces only"
    return None
