def two_pivots(weights: list[int]) -> int:
    n = len(weights)
    prefix = [0] * (n + 1)
    for k, w in enumerate(weights):
        prefix[k + 1] = prefix[k] + w
    total = prefix[n]
    seen: dict[tuple[int, int], int] = {}
    count = 0
    for j in range(1, n):
        i = j - 1
        key = (prefix[i], weights[i])
        seen[key] = seen.get(key, 0) + 1
        right = total - prefix[j + 1]
        count += seen.get((right, prefix[j] - 2 * right), 0)
    return count
