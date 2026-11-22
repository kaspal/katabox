def till_total(prices: list[str]) -> int:
    # Count in whole cents: "12.30" is 1230.
    return sum(int(p.replace(".", "")) for p in prices)
