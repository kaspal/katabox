"""Places without gaps per division, in plain Python."""


def solve(tables):
    by_div = {}
    for tid, name, div, points in tables["teams"]:
        by_div.setdefault(div, []).append((tid, name, points))
    rows = []
    for div in sorted(by_div):
        distinct = sorted({p for _, _, p in by_div[div]}, reverse=True)
        place = {p: i + 1 for i, p in enumerate(distinct)}
        for tid, name, points in sorted(by_div[div], key=lambda t: (place[t[2]], t[1], t[0])):
            rows.append([div, name, points, place[points]])
    return {"columns": ["division", "name", "points", "place"], "rows": rows}
