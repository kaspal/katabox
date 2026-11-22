def f(n):
    stack = [0]
    visited = 0
    while stack:
        node = stack.pop()
        visited += 1
        right = 2 * node + 2
        if right < n:
            stack.append(right)
        left = 2 * node + 1
        if left < n:
            stack.append(left)
    return visited
