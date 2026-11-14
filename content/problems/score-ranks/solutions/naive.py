import sys


# Correct but O(N·Q): for each question, scans every score and collects the distinct
# ones above it.
def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    n = int(tokens[0])
    scores = [int(x) for x in tokens[1 : 1 + n]]
    q = int(tokens[1 + n])
    answers: list[str] = []
    for t in tokens[2 + n : 2 + n + q]:
        x = int(t)
        if x not in scores:
            answers.append("0")
        else:
            above = {s for s in scores if s > x}
            answers.append(str(len(above) + 1))
    sys.stdout.write(" ".join(answers) + "\n")


main()
