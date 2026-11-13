def vowel_tally(text: str) -> int:
    count = 0
    for c in text:
        if c.lower() in "aeiou":
            count += 1
    return count
