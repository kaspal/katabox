Simulate the shop: for each job you need the lathe that frees up first. Scanning every lathe for it costs O(lathes) per job.
---
You only ever need the smallest "free at" time, then you change it. Which data structure gives the minimum quickly and accepts updates?
---
Keep a min-heap of the times at which each lathe becomes free (start with `min(lathes, n)` zeros). For each job, pop the smallest time `t` and push `t + job`. The answer is the largest time pushed.
