def is_mirror(text: str) -> bool:
    # Obviously correct: clean the text, then compare with its reverse.
    kept = [c.lower() for c in text if c.isascii() and c.isalnum()]
    return kept == kept[::-1]
