def product_mod(nums: list[int]) -> int:
    # Obviously correct: the exact product (Python integers never overflow), then the modulo.
    p = 1
    for x in nums:
        p *= x
    return p % 1_000_000_007
