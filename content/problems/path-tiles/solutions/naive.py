def tilings(n: int) -> int:
    # Correct but exponential: walk through every paving (an explicit stack, so long
    # paths run out of time instead of hitting the recursion limit).
    count = 0
    stack = [n]
    while stack:
        left = stack.pop()
        if left == 0:
            count += 1
            continue
        for p in (1, 2, 3):
            if p <= left:
                stack.append(left - p)
    return count % 1_000_000_007
