def temperature_swing(readings: list[int]) -> int:
    # Obviously correct: the largest difference between any two readings.
    return max(a - b for a in readings for b in readings)
