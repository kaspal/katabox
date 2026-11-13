DIGITS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"


def convert_base(number: str, source: int, target: int) -> str:
    # Obviously correct: Python's own parser, then the largest power of the target first.
    value = int(number, source)
    if value == 0:
        return "0"
    power = 1
    while power * target <= value:
        power *= target
    out = ""
    while power >= 1:
        out += DIGITS[value // power]
        value %= power
        power //= target
    return out
