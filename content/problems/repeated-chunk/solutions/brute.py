def longest_repeat(text: str) -> int:
    # Obviously correct (short texts): every length, longest first, with a set of pieces.
    n = len(text)
    for length in range(n - 1, 0, -1):
        seen = set()
        for i in range(n - length + 1):
            piece = text[i:i + length]
            if piece in seen:
                return length
            seen.add(piece)
    return 0
