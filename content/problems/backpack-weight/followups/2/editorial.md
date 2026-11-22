## Idea

Swap the roles of weight and worth. Let `S` be the sum of all worths (at most 10⁴) and
`lightest[v]` the least weight of a group of items worth exactly `v`, starting with
`lightest[0] = 0` and every other entry unreachable. Each item of weight `w` and worth
`x` updates `lightest[v] = min(lightest[v], lightest[v − x] + w)` for `v` from `S` down
to `x`; going downwards uses each item at most once, as in the base problem. Finally,
scan for the largest `v` whose `lightest[v]` fits in `capacity`; `v = 0` always fits.

The total weight of all items is at most 100 · 10⁷ = 10⁹, so every sum fits in a 32-bit
signed integer.

## Complexity

O(n · S) time, at most 100 · 10⁴ = 10⁶ steps, and O(S) extra memory, whatever the
capacity.
