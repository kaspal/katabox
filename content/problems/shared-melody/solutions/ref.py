def shared_melody(first: list[int], second: list[int]) -> int:
    m = len(second)
    prev = [0] * (m + 1)
    for x in first:
        cur = [0] * (m + 1)
        for j in range(1, m + 1):
            if x == second[j - 1]:
                cur[j] = prev[j - 1] + 1
            else:
                cur[j] = prev[j] if prev[j] > cur[j - 1] else cur[j - 1]
        prev = cur
    return prev[m]
