def validate(inp):
    text = inp["text"]
    if not isinstance(text, str):
        return "text is a string"
    if not 1 <= len(text) <= 10**5:
        return "1 ≤ text.length ≤ 10⁵"
    if not all(32 <= ord(c) <= 126 for c in text):
        return "text contains printable ASCII characters (codes 32 to 126)."
    return None
