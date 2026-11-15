"""The balance after each transaction, in plain Python."""


def solve(tables):
    rows = sorted((acct, day, tid, amount) for tid, acct, day, amount in tables["transactions"])
    out, balance, current = [], 0, None
    for acct, day, _, amount in rows:
        if acct != current:
            current, balance = acct, 0
        balance += amount
        out.append([acct, day, amount, balance])
    return {"columns": ["account", "day", "amount_cents", "balance_cents"], "rows": out}
