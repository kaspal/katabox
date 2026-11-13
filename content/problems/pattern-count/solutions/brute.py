def count_pattern(text: str, pattern: str) -> int:
    # Obviously correct: test every starting position with a slice.
    m = len(pattern)
    return sum(1 for i in range(len(text) - m + 1) if text[i:i + m] == pattern)
