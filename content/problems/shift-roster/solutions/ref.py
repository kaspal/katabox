import sys


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    n = int(tokens[0])
    names = tokens[1 : 1 + 2 * n : 2]
    hours = [int(h) for h in tokens[2 : 2 + 2 * n : 2]]
    total = sum(hours)
    out = [str(total)]
    for name, h in zip(names, hours):
        if h * n > total:  # h > total / n, in whole numbers
            out.append(f"{name.decode()} {h}")
    if len(out) == 1:
        out.append("none")
    sys.stdout.write("\n".join(out) + "\n")


main()
