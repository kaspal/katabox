def validate(inp):
    w, h = inp["width"], inp["height"]
    if type(w) is not int or type(h) is not int:
        return "width and height are integers"
    if not (1 <= w <= 10**12 and 1 <= h <= 10**12):
        return "1 ≤ width, height ≤ 10¹²"
    return None
