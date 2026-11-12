def running_medians(readings: list[int]) -> list[float]:
    # Obviously correct: sort each prefix and read off the middle.
    out = []
    for i in range(1, len(readings) + 1):
        s = sorted(readings[:i])
        out.append(float(s[i // 2]) if i % 2 else (s[i // 2 - 1] + s[i // 2]) / 2)
    return out
