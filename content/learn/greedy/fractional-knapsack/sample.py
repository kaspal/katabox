from math import gcd


def fractional(capacity, items):
    order = sorted(
        range(len(items)), key=lambda k: (-items[k][1] / items[k][0], k)
    )
    whole, room = 0, capacity
    for k in order:
        weight, value = items[k]
        if weight <= room:
            whole += value  # @step take
            room -= weight
        else:
            num, den = value * room, weight  # @step part
            g = gcd(num, den)
            return [whole + num // den, (num % den) // g, den // g]
    return [whole, 0, 1]
