import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    levels = list(map(int, data[1 : 1 + n]))
    # low[k][i]: the lowest of the 2^k readings starting at i.
    low = [levels]
    half = 1
    while 2 * half <= n:
        prev = low[-1]
        low.append(list(map(min, prev[:-half], prev[half:])))
        half *= 2
    q = int(data[1 + n])
    out: list[str] = []
    pos = 2 + n
    for _ in range(q):
        l, r = int(data[pos]) - 1, int(data[pos + 1]) - 1
        pos += 2
        k = (r - l + 1).bit_length() - 1
        row = low[k]
        a, b = row[l], row[r - (1 << k) + 1]
        out.append(str(a if a < b else b))
    sys.stdout.write("\n".join(out) + "\n")


main()
