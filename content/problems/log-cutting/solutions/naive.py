def cut_cost(length: int, marks: list[int]) -> int:
    # Correct but exponential: try every first cut recursively, remembering nothing.
    p = [0] + sorted(marks) + [length]

    def go(i, j):
        if j - i < 2:
            return 0
        return (p[j] - p[i]) + min(go(i, k) + go(k, j) for k in range(i + 1, j))

    return go(0, len(p) - 1)
