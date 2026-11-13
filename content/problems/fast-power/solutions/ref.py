P = 1_000_003


def power_mod(base: int, exponent: int) -> int:
    result, square = 1, base % P
    while exponent > 0:
        if exponent % 2 == 1:
            result = result * square % P
        square = square * square % P
        exponent //= 2
    return result
