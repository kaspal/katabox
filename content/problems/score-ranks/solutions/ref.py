import sys


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    n = int(tokens[0])
    distinct = sorted({int(x) for x in tokens[1 : 1 + n]}, reverse=True)
    rank = {score: j + 1 for j, score in enumerate(distinct)}
    q = int(tokens[1 + n])
    answers = [str(rank.get(int(x), 0)) for x in tokens[2 + n : 2 + n + q]]
    sys.stdout.write(" ".join(answers) + "\n")


main()
