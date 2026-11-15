# Obviously correct: follow the rules literally, looking at every job each time.
def main() -> None:
    n = int(input())
    jobs = [tuple(map(int, input().split())) for _ in range(n)]
    done = [False] * n
    order = []
    t = 0
    for _ in range(n):
        ready = [i for i in range(n) if not done[i] and jobs[i][0] <= t]
        if not ready:
            t = min(jobs[i][0] for i in range(n) if not done[i])
            ready = [i for i in range(n) if not done[i] and jobs[i][0] <= t]
        # Highest priority; then the earliest arrival; then the smallest number.
        pick = min(ready, key=lambda i: (-jobs[i][1], jobs[i][0], i))
        done[pick] = True
        order.append(pick + 1)
        t += jobs[pick][2]
    print(" ".join(map(str, order)))


main()
