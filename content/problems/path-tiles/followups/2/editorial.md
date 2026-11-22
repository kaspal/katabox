## Idea

The rule depends on the last piece, so keep it in the state. Let `a[i]` count the pavings
of length `i` that do not end in a long slab (with `a[0] = 1` for the empty paving) and
`b[i]` those that do:

- `b[i] = a[i − 3]`: a long slab may only follow a paving that does not end in one.
- `a[i] = (a[i − 1] + b[i − 1]) + (a[i − 2] + b[i − 2])`: a square or a short slab may
  follow anything.

The answer is `a[n] + b[n]`, all modulo 1 000 000 007. Only the last three lengths are
ever needed, so a few variables are enough.

## Complexity

O(n) time and O(1) extra memory (O(n) if you keep the arrays).
