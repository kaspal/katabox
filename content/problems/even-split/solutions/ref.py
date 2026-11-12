def settle(paid: list[float]) -> list[float]:
    share = sum(paid) / len(paid)
    return [p - share for p in paid]
