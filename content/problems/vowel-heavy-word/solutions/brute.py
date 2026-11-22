def most_vowels(words: list[str]) -> int:
    # Obviously correct: count each word's vowels, take the largest.
    return max(len([c for c in w if c in "aeiou"]) for w in words)
