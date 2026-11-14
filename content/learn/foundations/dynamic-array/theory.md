## The idea

A dynamic array (a Python `list`, a JavaScript array) keeps its values in a block of storage with room for `capacity` values, of which `size` are in use. Appending writes into the next free slot. When the block is full there is no room to grow in place, so the array allocates a new block of **double** the capacity, copies every value across and only then writes the new one. That copy is slow, but it happens so rarely that the cost **averaged over all appends** stays constant: this is amortised `O(1)`.

## Steps

1. Start with `capacity = 1`, `size = 0` and `copies = 0`.
2. For each value `v`, check: is `size == capacity`?
3. If it is full: double `capacity`, allocate the bigger storage and copy the `size` values across, one copy each.
4. Write `v` at position `size` and add one to `size`.
5. Watch the "writes per append" counter: writes plus copies, divided by the appends so far.

## Cost

| Measure | Cost |
|---|---|
| One append, usually | `O(1)`: a single write |
| One append that resizes | `O(n)`: every value is copied |
| `n` appends in total | `O(n)`: fewer than `2n` copies plus `n` writes |
| Amortised per append | `O(1)`: always below 3 writes |
| Extra space | `O(n)`: at most half the storage is empty |

## Pitfalls

- **Growing by a fixed amount.** Adding, say, 10 slots on each resize copies `10 + 20 + 30 + …` values: `O(n²)` in total, `O(n)` per append. The growth must be a **factor** (real libraries pick one such as 1.5 or 2), so that each copy pays for many cheap appends.
- **Amortised is not "always fast".** One append can still copy every value. In code with a deadline per operation, that single spike may matter; if you know the final size, allocate it up front.
- **Shrinking too eagerly.** Halving the storage as soon as it is half empty means that alternating one append and one removal at the boundary resizes every time. Shrink only when the array is a quarter full.

## When to use it

This is why appending to a list in a loop is fine: `n` appends cost `O(n)`, not `O(n²)`. More generally, amortised analysis lets you accept an occasional expensive step when the cheap ones pay for it, the same reasoning behind hash tables that resize and two-stack queues.

## Related topics

- [Big-O growth curves](learn:foundations/big-o-growth): `O(1)`, `O(n)` and `O(n²)` side by side.
- [Prefix sums](learn:foundations/prefix-sums): another case of paying once up front to make later work cheap.
- Hash tables (coming soon): they resize the same way.
