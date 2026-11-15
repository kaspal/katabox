"""Orders and revenue per day, in plain Python."""


def solve(tables):
    days = {}
    for _, placed, total in tables["orders"]:
        n, s = days.get(placed[:10], (0, 0))
        days[placed[:10]] = (n + 1, s + total)
    return {"columns": ["day", "orders", "revenue_cents"],
            "rows": [[d, n, s] for d, (n, s) in sorted(days.items())]}
