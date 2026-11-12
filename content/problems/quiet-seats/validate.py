def validate(inp):
    rows = inp["rows"]
    if not isinstance(rows, list) or not all(isinstance(r, str) for r in rows):
        return "rows is an array of strings"
    if not 1 <= len(rows) <= 400:
        return "1 ≤ rows.length ≤ 400"
    w = len(rows[0])
    if not 1 <= w <= 400 or any(len(r) != w for r in rows):
        return "1 ≤ rows[i].length ≤ 400, all rows the same length"
    if any(ch not in "#." for r in rows for ch in r):
        return "every character is # or ."
    return None
