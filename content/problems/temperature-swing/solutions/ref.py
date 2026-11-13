def temperature_swing(readings: list[int]) -> int:
    warmest = coldest = readings[0]
    for r in readings:
        if r > warmest:
            warmest = r
        if r < coldest:
            coldest = r
    return warmest - coldest
