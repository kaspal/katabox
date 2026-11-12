def days_until_warmer(temps: list[int]) -> list[int]:
    # Correct but O(n²) on long flat or cold spells: scan forward from every day.
    n = len(temps)
    wait = [0] * n
    for i in range(n):
        for j in range(i + 1, n):
            if temps[j] > temps[i]:
                wait[i] = j - i
                break
    return wait
