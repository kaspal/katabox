def balance_point(weights: list[int]) -> int:
    # Correct but O(n²): both sides summed from scratch for every index.
    n = len(weights)
    for i in range(n):
        left = 0
        for j in range(i):
            left += weights[j]
        right = 0
        for j in range(i + 1, n):
            right += weights[j]
        if left == right:
            return i
    return -1
