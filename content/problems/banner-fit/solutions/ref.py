import sys


def main() -> None:
    lines = sys.stdin.read().split()
    r, c = int(lines[0]), int(lines[1])
    # height[x]: how many free squares stand in column x, ending at the current row. Each
    # row then is a skyline, and the best banner whose bottom edge is on this row is the
    # largest rectangle under it: a stack of columns of increasing height.
    height = [0] * (c + 1)  # a 0 at the end empties the stack
    best = 0
    for row in lines[2 : 2 + r]:
        for x in range(c):
            height[x] = height[x] + 1 if row[x] == "." else 0
        stack: list[int] = []
        for x in range(c + 1):
            h = height[x]
            while stack and height[stack[-1]] >= h:
                top = height[stack.pop()]
                left = stack[-1] + 1 if stack else 0
                if top * (x - left) > best:
                    best = top * (x - left)
            stack.append(x)
    print(best)


main()
