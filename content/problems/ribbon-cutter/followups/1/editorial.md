## Idea

The base problem binary searched the answer with an O(n) check; repeating that for 10⁵
orders is too slow. Instead, compute `count[L]`, the number of pieces of length `L`, for
every `L` from 1 to `m = max(ribbons)`. Build `atMost[v]`, the number of rolls of length
at most `v`; then a roll yields a piece for each multiple of `L` it reaches, so
`count[L]` is the sum of `n − atMost[k·L − 1]` over the multiples `k·L ≤ m`. Since
`count` never increases, the answer to an order `need` is the number of lengths with
`count[L] ≥ need`, found by binary search (0 when even `count[1]` is too small).

## Complexity

O(m log m) time to fill `count` (the harmonic sum m/1 + m/2 + … + m/m), then
O(log m) per order; O(m) extra memory. Piece counts reach 10¹⁰, so store them and
compare them with 64-bit integers.
