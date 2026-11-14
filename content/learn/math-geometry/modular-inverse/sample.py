def mod_inverse(a, m):
    old_r, r = a % m, m
    old_x, x = 1, 0
    while r:
        q = old_r // r
        old_r, r = r, old_r - q * r  # @step divide
        old_x, x = x, old_x - q * x
    if old_r != 1:
        return [-1]
    return [old_x % m]
