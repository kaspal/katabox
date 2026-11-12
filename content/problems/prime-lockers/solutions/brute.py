def prime_counts(limits: list[int]) -> list[int]:
    # Obviously correct: count the numbers up to L with exactly two divisors.
    def is_prime(x):
        return x >= 2 and all(x % d for d in range(2, x))

    return [sum(1 for x in range(1, L + 1) if is_prime(x)) for L in limits]
