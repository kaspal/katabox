import sys


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    n = int(tokens[0])
    readings = list(map(int, tokens[1 : 1 + n]))
    total = sum(readings)
    rises = 0
    run = longest = 1
    prev = readings[0]
    for x in readings[1:]:
        if x == prev:
            run += 1
            if run > longest:
                longest = run
        else:
            if x > prev:
                rises += 1
            run = 1
        prev = x
    sys.stdout.write(f"{total} {rises} {longest}\n")


main()
