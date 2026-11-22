"""race-results constraints."""


def validate(inp):
    names, times = inp["names"], inp["times"]
    if not isinstance(names, list) or not isinstance(times, list) or len(names) != len(times):
        return "names.length = times.length"
    if not 1 <= len(names) <= 10**4:
        return "1 ≤ names.length ≤ 10⁴"
    if not all(isinstance(n, str) and 1 <= len(n) <= 10 and n.isascii() and n.isalnum() and n == n.lower() for n in names):
        return "names have 1 to 10 lowercase letters or digits"
    if len(set(names)) != len(names):
        return "names are all different"
    if not all(type(t) is int and 1 <= t <= 10**6 for t in times):
        return "1 ≤ times[i] ≤ 10⁶"
    return None
