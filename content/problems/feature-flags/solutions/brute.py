def users_with_all(users: list[int], queries: list[int]) -> list[int]:
    # Obviously correct: test every user for every question.
    return [sum(1 for u in users if u & q == q) for q in queries]
