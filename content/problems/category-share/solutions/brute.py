"""Each category's share of all sales, in plain Python."""
from decimal import ROUND_HALF_UP, Decimal


def solve(tables):
    totals = {}
    for _, category, amount in tables["sales"]:
        totals[category] = totals.get(category, 0) + amount
    grand = sum(totals.values())
    rows = []
    for category, total in totals.items():
        # As SQLite: a real division, then round() half away from zero.
        share = float(Decimal(100.0 * total / grand).quantize(Decimal("0.1"), ROUND_HALF_UP))
        rows.append((-share, category, total))
    rows.sort()
    return {"columns": ["category", "total_cents", "share_pct"],
            "rows": [[c, t, -s] for s, c, t in rows]}
