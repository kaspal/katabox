def leap_years(start: int, end: int) -> int:
    count = 0
    for y in range(start, end + 1):
        if y % 4 == 0 and (y % 100 != 0 or y % 400 == 0):
            count += 1
    return count
