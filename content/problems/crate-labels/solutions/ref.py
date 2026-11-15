import sys


def main() -> None:
    lines = sys.stdin.read().split()
    n = int(lines[0])
    total: dict[str, int] = {}
    for label in lines[1 : 1 + n]:
        for entry in label.split(";"):
            name, qty = entry.split(":")
            total[name] = total.get(name, 0) + int(qty)
    out = [f"{name} {total[name]}" for name in sorted(total)]
    sys.stdout.write("\n".join(out) + "\n")


main()
