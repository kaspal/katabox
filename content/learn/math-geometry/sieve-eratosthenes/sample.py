def sieve(n):
    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False
    p = 2
    while p * p <= n:
        if is_prime[p]:
            start = p * p  # @step prime
            for multiple in range(start, n + 1, p):
                is_prime[multiple] = False  # @step cross
        p += 1
    primes = [k for k in range(n + 1) if is_prime[k]]
    return [len(primes)] + primes
