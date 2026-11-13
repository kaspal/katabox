def trapped_rain(columns: list[int]) -> int:
    i, j = 0, len(columns) - 1
    left_max = right_max = water = 0
    while i <= j:
        left_max = max(left_max, columns[i])
        right_max = max(right_max, columns[j])
        if left_max <= right_max:
            water += left_max - columns[i]
            i += 1
        else:
            water += right_max - columns[j]
            j -= 1
    return water
