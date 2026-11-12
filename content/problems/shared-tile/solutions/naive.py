def largest_tile(width: int, height: int) -> int:
    # Correct but O(min(width, height)): try every size from the smaller side down.
    s = min(width, height)
    while width % s or height % s:
        s -= 1
    return s
