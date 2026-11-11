LETTERS = set("abcdefghijklmnopqrstuvwxyz")


def validate(inp):
    text = inp["text"]
    if not isinstance(text, str):
        return "text is a string"
    if not 1 <= len(text) <= 10**5:
        return "1 ≤ text.length ≤ 10⁵"
    if not set(text) <= LETTERS:
        return "text contains only lowercase English letters"
    return None
