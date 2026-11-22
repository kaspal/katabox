The base recurrence `f(i) = f(i − 1) + f(i − 2) + f(i − 3)` is still right, but a loop
of 10¹⁵ steps never finishes. How can you take many steps at once?
---
One step turns the triple `(f(i), f(i − 1), f(i − 2))` into `(f(i + 1), f(i), f(i − 1))`,
and that step is the same linear map every time: a fixed 3×3 matrix.
---
Taking `n` steps is the `n`-th power of that matrix. Compute it by repeated squaring
(square the matrix, and multiply it into the result when the current bit of `n` is 1),
reducing every entry modulo 1 000 003, then apply it to `(f(0), f(−1), f(−2)) = (1, 0, 0)`.
