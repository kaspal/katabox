"""change-machine--f2 constraints."""


def validate(inp):
    tokens, amount = inp["tokens"], inp["amount"]
    if not isinstance(tokens, list) or not all(type(t) is int for t in tokens) or type(amount) is not int:
        return "tokens is an array of integers; amount is an integer"
    if not 1 <= len(tokens) <= 100:
        return "1 ≤ tokens.length ≤ 100"
    if not all(1 <= t <= 10**4 for t in tokens):
        return "1 ≤ tokens[i] ≤ 10⁴"
    if not 0 <= amount <= 10**4:
        return "0 ≤ amount ≤ 10⁴"
    return None
