def generations(parent: list[int]) -> int:
    children: list[list[int]] = [[] for _ in parent]
    founder = 0
    for i, p in enumerate(parent):
        if p < 0:
            founder = i
        else:
            children[p].append(i)
    level = [founder]
    count = 0
    while level:
        count += 1
        level = [c for p in level for c in children[p]]
    return count
