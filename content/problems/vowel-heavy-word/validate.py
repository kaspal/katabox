"""vowel-heavy-word constraints."""


def validate(inp):
    w = inp["words"]
    if not isinstance(w, list) or not 1 <= len(w) <= 10**4:
        return "1 ≤ words.length ≤ 10⁴"
    if not all(isinstance(x, str) and 1 <= len(x) <= 20 and all("a" <= c <= "z" for c in x) for x in w):
        return "each word has 1 to 20 lowercase English letters"
    return None
