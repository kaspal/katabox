def groups(n: int, links: list[list[int]]) -> int:
    # Correct but O(n·m): give each island the smallest label among its ferry
    # neighbours, pass after pass, until nothing changes.
    label = list(range(n))
    changed = True
    while changed:
        changed = False
        for a, b in links:
            low = min(label[a], label[b])
            if label[a] != low or label[b] != low:
                label[a] = label[b] = low
                changed = True
    return len(set(label))
