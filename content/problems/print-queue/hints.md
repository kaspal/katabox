Simulate the printer with a clock `t`. The input is not in arrival order, so first sort the jobs by arrival (keeping each job's number). Then the jobs that have arrived by time `t` are always a prefix of that sorted list.
---
Scanning every waiting job at each pick is correct but slow: in a busy office tens of thousands of jobs wait at once. Keep the waiting jobs in a **priority queue** (heap) ordered by the printer's rule, so each pick costs O(log n).
---
Order the heap by the key `(−priority, arrival, number)`, smallest first. Loop until all jobs are printed: if the heap is empty and the next job arrives after `t`, jump `t` to its arrival; push every job with `arrival ≤ t`; pop the top job, record its number and add its pages to `t`.
