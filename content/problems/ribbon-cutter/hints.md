For a fixed length `L`, how many pieces do you get? How does that number change as `L` grows?
---
The count `sum(r // L)` never increases when `L` grows. So the lengths that work form a prefix 1, 2, …, answer. Trying every length one by one can mean 10⁷ tries.
---
Binary search `L` between 1 and the longest roll: if length `mid` gives at least `pieces` pieces, the answer is `mid` or more; otherwise it is less. Check first that length 1 works at all.
