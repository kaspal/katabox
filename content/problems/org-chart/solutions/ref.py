import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    team: list[list[int]] = [[] for _ in range(n + 1)]
    for i in range(2, n + 1):
        team[int(data[i - 1])].append(i)
    # Number the employees in depth-first order, with an explicit stack (a chart can be
    # 10⁵ levels deep: far too deep for recursion). first[x] is x's number; last[x] the
    # largest number in x's team, so the team is exactly first[x]..last[x].
    first = [0] * (n + 1)
    last = [0] * (n + 1)
    clock = 0
    stack = [(1, False)]
    while stack:
        x, done = stack.pop()
        if done:
            last[x] = clock
            continue
        clock += 1
        first[x] = clock
        stack.append((x, True))
        for y in team[x]:
            stack.append((y, False))
    q = int(data[n])
    out: list[str] = []
    pos = n + 1
    for _ in range(q):
        a, b = int(data[pos]), int(data[pos + 1])
        pos += 2
        out.append("YES" if a != b and first[a] <= first[b] <= last[a] else "NO")
    sys.stdout.write("\n".join(out) + "\n")


main()
