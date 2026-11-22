Try an `x` that is larger than every element. What index should come back, and can the code ever return it?
---
The loop body is a correct lower-bound search. Look at the range it starts with.
---
`hi` starts at `len(sorted) - 1`, so the answer can never be `len(sorted)`. Start with `hi = len(sorted)` (in TypeScript, `sorted.length`).
