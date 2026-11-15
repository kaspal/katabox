"""The second-highest distinct amount per region, in plain Python."""


def solve(tables):
    amounts = {}
    for _, region, amount in tables["sales"]:
        amounts.setdefault(region, set()).add(amount)
    rows = []
    for region in sorted(amounts):
        distinct = sorted(amounts[region], reverse=True)
        rows.append([region, distinct[1] if len(distinct) > 1 else None])
    return {"columns": ["region", "runner_up_cents"], "rows": rows}
