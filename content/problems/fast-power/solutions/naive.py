def power_mod(base: int, exponent: int) -> int:
    # Correct but O(exponent): multiply one factor at a time.
    result = 1
    for _ in range(exponent):
        result = result * base % 1_000_003
    return result
