import sys

MOD = 1_000_003


def ways(s: bytes) -> int:
    # before = ways of the prefix one digit shorter, last = ways of the current prefix.
    before, last = 1, (0 if s[0] == 48 else 1)
    for i in range(1, len(s)):
        c, p = s[i], s[i - 1]
        cur = last if c != 48 else 0
        if p == 49 or (p == 50 and c <= 54):  # "1x" or "20".."26"
            cur += before
        before, last = last, cur % MOD
    return last


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    t = int(tokens[0])
    out = [str(ways(tokens[1 + j][1:])) for j in range(t)]
    sys.stdout.write("\n".join(out) + "\n")


main()
