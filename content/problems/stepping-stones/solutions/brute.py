def can_cross(jumps: list[int]) -> bool:
    # Obviously correct: a search over stones, trying every jump length.
    n = len(jumps)
    seen, stack = {0}, [0]
    while stack:
        i = stack.pop()
        for k in range(1, jumps[i] + 1):
            if i + k < n and i + k not in seen:
                seen.add(i + k)
                stack.append(i + k)
    return n - 1 in seen
