import sys
from math import gcd


def inverse(a: int, mod: int) -> int:
    """a⁻¹ modulo mod (gcd(a, mod) = 1) by the extended Euclidean algorithm; 0 when mod = 1."""
    old_r, r = a, mod
    old_s, s = 1, 0
    while r:
        q = old_r // r
        old_r, r = r, old_r - q * r
        old_s, s = s, old_s - q * s
    return old_s % mod


def main() -> None:
    data = sys.stdin.buffer.read().split()
    t = int(data[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        n = int(data[pos])
        pos += 1
        x, big = 0, 1  # the lines so far all leave at minutes x + big·k
        ok = True
        for _ in range(n):
            r, m = int(data[pos]), int(data[pos + 1])
            pos += 2
            if not ok:
                continue
            g = gcd(big, m)
            if (r - x) % g:
                ok = False
                continue
            step = m // g
            k = (r - x) // g % step * inverse(big // g % step, step) % step
            x += big * k
            big *= step
        out.append(str(x) if ok else "-1")
    sys.stdout.write("\n".join(out) + "\n")


main()
