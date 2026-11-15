# Correct but slow: a plain list of waiting jobs, scanned in full for every pick, O(n²).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    jobs = [(int(data[1 + 3 * i]), int(data[2 + 3 * i]), int(data[3 + 3 * i]), i + 1) for i in range(n)]
    jobs.sort()  # by arrival, then number
    waiting: list[tuple[int, int, int, int]] = []
    order: list[int] = []
    t = 0
    nxt = 0
    while len(order) < n:
        if not waiting and jobs[nxt][0] > t:
            t = jobs[nxt][0]
        while nxt < n and jobs[nxt][0] <= t:
            waiting.append(jobs[nxt])
            nxt += 1
        best = 0
        for k in range(1, len(waiting)):
            a, p, _, i = waiting[k]
            ba, bp, _, bi = waiting[best]
            if (-p, a, i) < (-bp, ba, bi):
                best = k
        a, p, g, i = waiting.pop(best)
        order.append(i)
        t += g
    sys.stdout.write(" ".join(map(str, order)) + "\n")


main()
