def shared_melody(first: list[int], second: list[int]) -> int:
    # Correct but exponential: recurse on both choices without remembering results.
    def go(i, j):
        if i == len(first) or j == len(second):
            return 0
        if first[i] == second[j]:
            return 1 + go(i + 1, j + 1)
        return max(go(i + 1, j), go(i, j + 1))

    return go(0, 0)
