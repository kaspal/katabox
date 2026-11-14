from collections import deque


def window_max(a, k):
    dq, out = deque(), []
    for i in range(len(a)):
        while dq and a[dq[-1]] <= a[i]:
            dq.pop()  # @step pop-back
        dq.append(i)  # @step push-back
        if dq[0] <= i - k:
            dq.popleft()  # @step pop-front
        if i >= k - 1:
            out.append(a[dq[0]])  # @step emit
    return out
