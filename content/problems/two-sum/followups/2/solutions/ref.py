def closest_sum(nums: list[int], target: int) -> int:
    a = sorted(nums)
    i, j = 0, len(a) - 1
    best = a[0] + a[1]
    while i < j:
        s = a[i] + a[j]
        if abs(s - target) < abs(best - target) or (abs(s - target) == abs(best - target) and s < best):
            best = s
        if s < target:
            i += 1
        elif s > target:
            j -= 1
        else:
            return s
    return best
