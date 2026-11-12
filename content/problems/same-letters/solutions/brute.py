def same_letters(a: str, b: str) -> bool:
    # Obviously correct: the same letters in sorted order.
    return sorted(a) == sorted(b)
