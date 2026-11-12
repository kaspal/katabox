Think of the steps as nodes and each rule `[a, b]` as an arrow `a → b`. A step is ready when every arrow into it comes from a finished step.
---
Count, for every step, how many unfinished steps it is waiting for. When a step is done, lower the count of every step it points to; those reaching zero become ready.
---
Keep the ready steps in a min-heap so the lowest number comes out first (Kahn's algorithm). If the heap empties before all `n` steps are done, there is a cycle: return `[]`.
