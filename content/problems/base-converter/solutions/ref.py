DIGITS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"


def convert_base(number: str, source: int, target: int) -> str:
    value = 0
    for ch in number:
        value = value * source + DIGITS.index(ch)
    if value == 0:
        return "0"
    out = []
    while value > 0:
        value, d = divmod(value, target)
        out.append(DIGITS[d])
    return "".join(reversed(out))
