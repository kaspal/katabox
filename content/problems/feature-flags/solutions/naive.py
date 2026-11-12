def users_with_all(users: list[int], queries: list[int]) -> list[int]:
    # Correct but O(n·q): test every user for every question.
    out = []
    for q in queries:
        c = 0
        for u in users:
            if u & q == q:
                c += 1
        out.append(c)
    return out
