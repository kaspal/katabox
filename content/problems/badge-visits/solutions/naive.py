import sys


# Correct but O(N·Q): recounts the distinct ids of each prefix from scratch.
def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    n = int(tokens[0])
    scans = [int(x) for x in tokens[1 : 1 + n]]
    q = int(tokens[1 + n])
    out: list[str] = []
    for j in range(q):
        p = int(tokens[2 + n + j])
        out.append(str(len(set(scans[:p]))))
    sys.stdout.write("\n".join(out) + "\n")


main()
