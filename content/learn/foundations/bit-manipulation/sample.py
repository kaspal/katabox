def bit_ops(x, y):
    results = [x & y]  # @step and
    results.append(x | y)  # @step or
    results.append(x ^ y)  # @step xor
    results.append(x << 1)  # @step left
    results.append(x >> 1)  # @step right
    count = 0
    while x != 0:
        x &= x - 1  # @step clear
        count += 1
    results.append(count)  # @step count
    return results
