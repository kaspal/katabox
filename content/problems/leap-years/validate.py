def validate(inp):
    a, b = inp["start"], inp["end"]
    if type(a) is not int or type(b) is not int:
        return "start and end are integers"
    if not 1 <= a <= b <= 9999:
        return "1 ≤ start ≤ end ≤ 9999"
    return None
