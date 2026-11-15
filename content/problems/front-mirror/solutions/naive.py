# Correct but slow: check prefixes from the longest down, letter by letter, O(n²).
import sys


def main() -> None:
    s = sys.stdin.read().split()[0]
    n = len(s)
    length = 1
    for cand in range(n, 0, -1):
        i, j = 0, cand - 1
        while i < j and s[i] == s[j]:
            i += 1
            j -= 1
        if i >= j:
            length = cand
            break
    sys.stdout.write(s[length:][::-1] + s + "\n")


main()
