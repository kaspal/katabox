def vowel_tally(text: str) -> int:
    # Obviously correct: count each of the ten vowel characters separately.
    return sum(text.count(v) for v in "aeiouAEIOU")
