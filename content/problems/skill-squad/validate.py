def validate(inp):
    m, p = inp["skills"], inp["people"]
    if type(m) is not int or not 1 <= m <= 14:
        return "1 ≤ skills ≤ 14"
    if not isinstance(p, list) or not 1 <= len(p) <= 60:
        return "1 ≤ people.length ≤ 60"
    if not all(type(v) is int and 0 <= v < 1 << m for v in p):
        return "0 ≤ people[i] < 2^skills"
    return None
