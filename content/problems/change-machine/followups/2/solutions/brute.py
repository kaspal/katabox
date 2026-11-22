from itertools import combinations


def fewest_tokens(tokens: list[int], amount: int) -> int:
    # Obviously correct for few tokens: try every group, smallest groups first.
    for k in range(len(tokens) + 1):
        if any(sum(group) == amount for group in combinations(tokens, k)):
            return k
    return -1
