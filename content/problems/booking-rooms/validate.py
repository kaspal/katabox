def validate(inp):
    meetings = inp["meetings"]
    if not isinstance(meetings, list):
        return "meetings is a 2-D array of integers"
    if not 1 <= len(meetings) <= 2 * 10**4:
        return "1 ≤ meetings.length ≤ 2·10⁴"
    for m in meetings:
        if not (isinstance(m, list) and len(m) == 2 and all(type(x) is int for x in m)
                and 0 <= m[0] < m[1] <= 10**9):
            return "meetings[i] = [start, end] with 0 ≤ start < end ≤ 10⁹"
    return None
