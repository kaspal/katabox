MOD = 1_000_000_007


def product_mod(nums: list[int]) -> int:
    acc = 1.0
    for x in nums:
        acc = acc * x % MOD
    return int(acc)
