def most_talks(talks: list[list[int]]) -> int:
    count, free = 0, -1
    for start, end in sorted(talks, key=lambda t: t[1]):
        if start >= free:
            count += 1
            free = end
    return count
