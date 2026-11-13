def bit_counts(n: int) -> list[int]:
    # Obviously correct: count the 1s in each binary string.
    return [bin(i).count("1") for i in range(n + 1)]
