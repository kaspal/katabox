def till_total(prices: list[str]) -> int:
    total = 0.0
    for p in prices:
        total += float(p)
    return int(total * 100)
