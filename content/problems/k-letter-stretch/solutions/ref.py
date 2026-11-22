def longest_with_k_letters(text: str, k: int) -> int:
    count: dict[str, int] = {}
    best = start = 0
    for end, c in enumerate(text):
        count[c] = count.get(c, 0) + 1
        while len(count) > k:
            s = text[start]
            count[s] -= 1
            if count[s] == 0:
                del count[s]
            start += 1
        best = max(best, end - start + 1)
    return best
