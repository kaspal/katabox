def same_letters(a: str, b: str) -> bool:
    if len(a) != len(b):
        return False
    counts = [0] * 26
    for x, y in zip(a, b):
        counts[ord(x) - 97] += 1
        counts[ord(y) - 97] -= 1
    return all(c == 0 for c in counts)
