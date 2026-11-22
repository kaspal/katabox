def days_until_warmer_around(temps: list[int]) -> list[int]:
    # Obviously correct: walk forward around the year from each day.
    n = len(temps)
    return [next((d for d in range(1, n) if temps[(i + d) % n] > temps[i]), 0) for i in range(n)]
