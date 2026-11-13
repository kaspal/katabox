def steep_drops(prices: list[int]) -> int:
    count = 0

    def sort(a):
        nonlocal count
        if len(a) < 2:
            return a
        mid = len(a) // 2
        left, right = sort(a[:mid]), sort(a[mid:])
        j, m = 0, len(right)
        for x in left:
            while j < m and x > 2 * right[j]:
                j += 1
            count += j
        return sorted(left + right)

    sort(prices)
    return count
