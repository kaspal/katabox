import sys

LOWER = b"abcdefghijklmnopqrstuvwxyz"
UPPER = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"


def main() -> None:
    first, _, text = sys.stdin.buffer.read().partition(b"\n")
    s = int(first)

    # The encoded letter at position i decodes to the letter at position i - s.
    def back(alphabet: bytes) -> bytes:
        return alphabet[26 - s :] + alphabet[: 26 - s]

    table = bytes.maketrans(LOWER + UPPER, back(LOWER) + back(UPPER))
    sys.stdout.buffer.write(text.translate(table))


main()
