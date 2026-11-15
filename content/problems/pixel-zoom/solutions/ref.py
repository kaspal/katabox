import sys


def main() -> None:
    lines = sys.stdin.read().split("\n")
    r, c, k = map(int, lines[0].split())
    out: list[str] = []
    for row in lines[1 : 1 + r]:
        wide = "".join(ch * k for ch in row.strip())
        out.extend([wide] * k)
    sys.stdout.write("\n".join(out) + "\n")


main()
