"""Sales per year and quarter, in plain Python."""


def solve(tables):
    years = {}
    for _, day, amount in tables["sales"]:
        year, month = int(day[:4]), int(day[5:7])
        q = years.setdefault(year, [0, 0, 0, 0])
        q[(month - 1) // 3] += amount
    return {"columns": ["year", "q1", "q2", "q3", "q4"],
            "rows": [[y] + q for y, q in sorted(years.items())]}
