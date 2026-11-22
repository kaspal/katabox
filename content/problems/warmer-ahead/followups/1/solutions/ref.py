def days_until_warmer_around(temps: list[int]) -> list[int]:
    n = len(temps)
    wait = [0] * n
    stack: list[int] = []  # days still waiting, colder on top
    for k in range(2 * n):
        i = k % n
        while stack and temps[stack[-1]] < temps[i]:
            d = stack.pop()
            wait[d] = (i - d) % n or n
        if k < n:
            stack.append(i)
    return wait
