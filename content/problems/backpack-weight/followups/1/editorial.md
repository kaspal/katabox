## Idea

Reduce the problem to the base one. For an item with `c` copies, cut them into bundles of
1, 2, 4, ..., 2ᵏ⁻¹ copies, as long as they fit in `c`, and one last bundle with the rest.
Every number of copies from 0 to `c` is the sum of some of these bundles, and no choice of
bundles exceeds `c`. So packing each bundle at most once (a 0/1 item with the bundle's
total weight and worth) allows exactly the choices the problem allows.

Then run the base table: `best[cap]` is the best worth within weight `cap`, and each
bundle of weight `W` and worth `V` updates `best[cap] = max(best[cap], best[cap − W] + V)`
for `cap` from `capacity` down to `W`.

## Complexity

O(capacity · Σ log counts[i]) time: with at most 7 bundles per item, about
5000 · 50 · 7 ≈ 1.75·10⁶ steps. O(capacity) extra memory.
