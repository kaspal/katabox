def best_xor_pair(codes: list[int]) -> int:
    # The trie's greedy, done with prefix sets: fix the answer bit by bit from the top.
    best = 0
    for bit in range(19, -1, -1):
        prefixes = {c >> bit for c in codes}
        candidate = (best << 1) | 1
        best = candidate if any(candidate ^ p in prefixes for p in prefixes) else best << 1
    return best
