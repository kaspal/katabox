import sys


# Correct but O(label length) per check: reads the packed label from its start, run by
# run, every time, until the running total reaches k.
def char_at(label: bytes, k: int) -> str:
    count = 0
    total = 0
    for b in label:
        if 48 <= b <= 57:
            count = count * 10 + b - 48
        else:
            total += count
            if total >= k:
                return chr(b)
            count = 0
    raise ValueError("k is past the end of the label")


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    n_labels = int(tokens[0])
    labels = tokens[1 : 1 + n_labels]
    q = int(tokens[1 + n_labels])
    pos = 2 + n_labels
    out: list[str] = []
    for _ in range(q):
        i, k = int(tokens[pos]), int(tokens[pos + 1])
        pos += 2
        out.append(char_at(labels[i - 1], k))
    sys.stdout.write("\n".join(out) + "\n")


main()
