def plantings(length: int, widths: list[int]) -> int:
    # Correct but exponential: walk through every planting (with an explicit stack,
    # so long beds do not hit the recursion limit).
    count = 0
    stack = [(length, -1)]
    while stack:
        left, last = stack.pop()
        if left == 0:
            count += 1
            continue
        for k, w in enumerate(widths):
            if k != last and w <= left:
                stack.append((left - w, k))
    return count % 1_000_000_007
