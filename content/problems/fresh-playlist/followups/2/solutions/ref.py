def longest_rotation(plays: list[int], k: int) -> int:
    count: dict[int, int] = {}
    best = start = 0
    for end, song in enumerate(plays):
        count[song] = count.get(song, 0) + 1
        while len(count) > k:
            s = plays[start]
            count[s] -= 1
            if count[s] == 0:
                del count[s]
            start += 1
        best = max(best, end - start + 1)
    return best
