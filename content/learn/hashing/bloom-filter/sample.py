M = 16
HASHES = [(1, 0), (7, 3), (11, 5)]


def positions(key):
    return [(key * a + b) % M for a, b in HASHES]


def run(ops):
    bits, out = [0] * M, []
    for op in ops:
        if op[0] == "add":
            for p in positions(op[1]):
                bits[p] = 1  # @step set
        else:
            maybe = 1
            for p in positions(op[1]):
                if bits[p] == 0:  # @step check
                    maybe = 0
                    break
            out.append(maybe)  # @step answer
    return out
