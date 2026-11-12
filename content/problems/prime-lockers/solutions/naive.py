def prime_counts(limits: list[int]) -> list[int]:
    # Correct but slow: for every question, trial division for every number up to L.
    def is_prime(x):
        if x < 2:
            return False
        d = 2
        while d * d <= x:
            if x % d == 0:
                return False
            d += 1
        return True

    return [sum(1 for x in range(2, L + 1) if is_prime(x)) for L in limits]
