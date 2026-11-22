def most_vowels(words: list[str]) -> int:
    best = 0
    for w in words:
        count = sum(1 for c in w if c in "aeiou")
        best = max(best, count)
    return best
