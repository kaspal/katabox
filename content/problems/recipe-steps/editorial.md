## Approach: Kahn's algorithm with a min-heap

Treat each rule `[a, b]` as an arrow `a → b`, and let `waiting[b]` count the arrows into `b`. A step is ready when its count is 0.

1. Put every step with count 0 in a min-heap.
2. Repeatedly take the smallest ready step, append it to the answer, and lower the count of each step it points to. A count reaching 0 puts that step in the heap.
3. If fewer than `n` steps came out, the remaining steps wait on each other in a cycle: return `[]`.

The heap is what makes the order exactly the cook's: at every moment the cook does the lowest-numbered ready step, and that is the heap's minimum. A repeated rule adds the same arrow twice; it is counted twice and lowered twice, so nothing changes.

```
waiting[b] += 1 for each [a, b]; out[a].append(b)
heap = steps with waiting == 0
while heap:
    s = pop smallest; order.append(s)
    for t in out[s]:
        waiting[t] -= 1
        if waiting[t] == 0: push t
return order if len(order) == n else []
```

### Walkthrough

`n = 4`, rules 0 → 2, 1 → 2, 3 → 1. Counts: step 1 waits for 1, step 2 for 2. The heap starts with {0, 3}: take 0 (step 2 now waits for 1), take 3 (step 1 is ready), take 1 (step 2 is ready), take 2. Answer `[0, 3, 1, 2]`.

**Complexity:** O((n + m) log n) time and O(n + m) space. Rescanning every rule and every step to find the next ready one is O(n · (n + m)).
