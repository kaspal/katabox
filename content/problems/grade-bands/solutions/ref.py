def grade_bands(scores: list[int]) -> list[str]:
    bands = []
    for s in scores:
        if s >= 90:
            bands.append("A")
        elif s >= 80:
            bands.append("B")
        elif s >= 70:
            bands.append("C")
        elif s >= 60:
            bands.append("D")
        else:
            bands.append("F")
    return bands
