import heapq
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    jobs = [(int(data[1 + 3 * i]), int(data[2 + 3 * i]), int(data[3 + 3 * i]), i + 1) for i in range(n)]
    jobs.sort()  # by arrival, then number
    # The waiting jobs in a heap ordered by the printer's rule: highest priority, then
    # earliest arrival, then smallest number.
    waiting: list[tuple[int, int, int, int]] = []
    order: list[str] = []
    t = 0
    nxt = 0
    while len(order) < n:
        if not waiting and jobs[nxt][0] > t:
            t = jobs[nxt][0]  # nothing to print: wait for the next job
        while nxt < n and jobs[nxt][0] <= t:
            a, p, g, i = jobs[nxt]
            heapq.heappush(waiting, (-p, a, i, g))
            nxt += 1
        _, _, i, g = heapq.heappop(waiting)
        order.append(str(i))
        t += g
    sys.stdout.write(" ".join(order) + "\n")


main()
