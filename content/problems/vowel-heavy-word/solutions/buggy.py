def most_vowels(words: list[str]) -> int:
    for w in words:
        best = 0
        count = sum(1 for c in w if c in "aeiou")
        best = max(best, count)
    return best
