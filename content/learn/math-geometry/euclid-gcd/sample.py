def gcd_steps(a, b):
    steps = 0
    while b:
        a, b = b, a % b  # @step divide
        steps += 1
    return [a, steps]
