def validate(inp):
    length, marks = inp["length"], inp["marks"]
    if type(length) is not int or not isinstance(marks, list) or not all(type(m) is int for m in marks):
        return "length is an integer; marks is a list of integers"
    if not 2 <= length <= 10**6:
        return "2 ≤ length ≤ 10⁶"
    if not 1 <= len(marks) <= 100:
        return "1 ≤ marks.length ≤ 100"
    if not all(0 < m < length for m in marks) or len(set(marks)) != len(marks):
        return "0 < marks[i] < length, all different"
    return None
