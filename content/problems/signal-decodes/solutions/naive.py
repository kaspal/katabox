import sys

MOD = 1_000_003


# Correct but exponential: the plain "one digit or two digits" recursion without memo,
# run with an explicit stack (so long messages cannot overflow the call stack). It
# reaches the end once per reading, so it visits every reading one by one.
def ways(s: bytes) -> int:
    n = len(s)
    count = 0
    stack = [0]
    while stack:
        i = stack.pop()
        if i == n:
            count = (count + 1) % MOD
            continue
        c = s[i]
        if c == 48:  # a piece never starts with 0
            continue
        stack.append(i + 1)
        if i + 1 < n and (c == 49 or (c == 50 and s[i + 1] <= 54)):
            stack.append(i + 2)
    return count


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    t = int(tokens[0])
    out = [str(ways(tokens[1 + j][1:])) for j in range(t)]
    sys.stdout.write("\n".join(out) + "\n")


main()
