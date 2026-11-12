def shop(ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: keep a log of every unit movement and add it up each time.
    log = []
    reports = []
    for op, a in zip(ops, args):
        have = sum(d for item, d in log if item == a[0])
        if op == "stock":
            log.append((a[0], a[1]))
        elif op == "sell":
            sold = a[1] if a[1] <= have else have
            log.append((a[0], -sold))
            reports.append(sold)
        else:
            reports.append(have)
    return reports
