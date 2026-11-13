def mirror_cuts(text: str) -> int:
    # Correct but exponential: try every first piece recursively, remembering nothing.
    def go(start):
        if start == len(text):
            return -1
        best = len(text)
        for end in range(start + 1, len(text) + 1):
            piece = text[start:end]
            if piece == piece[::-1]:
                best = min(best, 1 + go(end))
        return best

    return go(0)
