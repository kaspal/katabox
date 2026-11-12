import re

NAME = re.compile(r"[a-z]{1,10}")


def validate(inp):
    names, times = inp["names"], inp["times"]
    if not isinstance(names, list) or not isinstance(times, list):
        return "names and times are arrays"
    if not 1 <= len(names) <= 10**4 or len(times) != len(names):
        return "1 ≤ names.length ≤ 10⁴ and times.length = names.length"
    if not all(isinstance(s, str) and NAME.fullmatch(s) for s in names):
        return "names have 1 to 10 lowercase English letters"
    if len(set(names)) != len(names):
        return "names are all different"
    if not all(type(t) is int and 1 <= t <= 10**5 for t in times):
        return "1 ≤ times[i] ≤ 10⁵"
    return None
