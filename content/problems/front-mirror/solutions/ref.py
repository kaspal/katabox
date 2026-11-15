import sys


def main() -> None:
    s = sys.stdin.read().split()[0]
    # The longest palindrome at the start of s is the longest prefix of s that is also a
    # suffix of reversed(s). The prefix function of s + "#" + reversed(s) gives exactly
    # that at its last position ("#" stops a match from running across the middle).
    t = s + "#" + s[::-1]
    pi = [0] * len(t)
    for i in range(1, len(t)):
        k = pi[i - 1]
        while k and t[i] != t[k]:
            k = pi[k - 1]
        if t[i] == t[k]:
            k += 1
        pi[i] = k
    keep = pi[-1]
    sys.stdout.write(s[keep:][::-1] + s + "\n")


main()
