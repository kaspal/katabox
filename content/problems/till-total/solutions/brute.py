def till_total(prices: list[str]) -> int:
    # Obviously correct: euros and cents read separately.
    total = 0
    for p in prices:
        euros, cents = p.split(".")
        total += int(euros) * 100 + int(cents)
    return total
