def validate(inp):
    b, e = inp["base"], inp["exponent"]
    if type(b) is not int or type(e) is not int:
        return "base and exponent are integers"
    if not (0 <= b <= 10**9 and 0 <= e <= 10**15):
        return "0 ≤ base ≤ 10⁹, 0 ≤ exponent ≤ 10¹⁵"
    return None
