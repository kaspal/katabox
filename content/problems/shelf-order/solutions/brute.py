import re


def natural_order(labels: list[str]) -> list[str]:
    # Obviously correct: insertion sort (stable) with a comparison that follows the
    # rules literally, using Python's big integers for digit chunks.
    def chunks(s):
        return re.findall(r"[0-9]+|[a-z]+", s)

    def less(a, b):
        ca, cb = chunks(a), chunks(b)
        for x, y in zip(ca, cb):
            dx, dy = x[0].isdigit(), y[0].isdigit()
            if dx != dy:
                return dx  # a digit chunk comes first
            vx, vy = (int(x), int(y)) if dx else (x, y)
            if vx != vy:
                return vx < vy
        return len(ca) < len(cb)

    out = []
    for label in labels:
        k = len(out)
        while k > 0 and less(label, out[k - 1]):
            k -= 1
        out.insert(k, label)
    return out
