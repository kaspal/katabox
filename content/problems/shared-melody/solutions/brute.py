from itertools import combinations


def shared_melody(first: list[int], second: list[int]) -> int:
    # Obviously correct (small inputs): the longest subsequence of first that is also a
    # subsequence of second.
    def is_sub(seq, of):
        it = iter(of)
        return all(any(x == y for y in it) for x in seq)

    for k in range(len(first), 0, -1):
        if any(is_sub([first[i] for i in idx], second) for idx in combinations(range(len(first)), k)):
            return k
    return 0
