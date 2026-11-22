def fresh_with_encores(plays: list[int]) -> int:
    count: dict[int, int] = {}
    best = start = 0
    for end, song in enumerate(plays):
        count[song] = count.get(song, 0) + 1
        while count[song] > 2:
            count[plays[start]] -= 1
            start += 1
        best = max(best, end - start + 1)
    return best
