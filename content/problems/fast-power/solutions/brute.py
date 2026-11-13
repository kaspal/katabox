def power_mod(base: int, exponent: int) -> int:
    # Obviously correct: Python's built-in modular power.
    return pow(base, exponent, 1_000_003)
