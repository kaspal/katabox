# Obviously correct: for every letter, search the plain letter whose encoding gives it.
import sys

LOWER = "abcdefghijklmnopqrstuvwxyz"
UPPER = LOWER.upper()


def main() -> None:
    first, _, text = sys.stdin.read().partition("\n")
    s = int(first)
    out = []
    for ch in text:
        for alphabet in (LOWER, UPPER):
            if ch in alphabet:
                for i, plain in enumerate(alphabet):
                    if alphabet[(i + s) % 26] == ch:
                        out.append(plain)
                        break
                break
        else:
            out.append(ch)
    sys.stdout.write("".join(out))


main()
