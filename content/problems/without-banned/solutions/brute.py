def without_banned(items: list[int], banned: list[int]) -> list[int]:
    # Obviously correct: copy every item that is not banned.
    out = []
    for x in items:
        if not any(x == b for b in banned):
            out.append(x)
    return out
