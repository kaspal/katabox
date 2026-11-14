## The idea

A **max-heap** is an array read as a binary tree: the children of position `i` are `2i + 1` and `2i + 2`, and every parent is at least as big as its children, so the biggest value sits at the root, `a[0]`. Heap sort first turns the array into a max-heap. Then it swaps the root to the end, shrinks the heap by one and repairs it. Each round puts the next biggest value in its final place, filling the array from the right.

## Steps

1. Build the heap: for each parent from the last one (`n // 2 - 1`) back to the root, **sift it down**.
2. To sift down, pick the bigger child. If the parent is smaller, swap them and keep going from the child's place; otherwise stop.
3. Swap the root (the biggest value) with the last value of the heap, `a[end]`. Position `end` is now final.
4. The heap now ends just before `end`. Sift the new root down to repair it.
5. Repeat until the heap holds one value.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n log n)` |
| Average | `O(n log n)` |
| Worst | `O(n log n)` |
| Extra space | `O(1)` (sifting is a loop, not recursion) |
| Stable | No: the root jumps to the end, over equal values |
| In place | Yes |

## Pitfalls

- **Children past the end.** A parent may have one child or none. Check `child < size` before reading a child, and `child + 1 < size` before comparing the two.
- **Sifting with the wrong size.** In the second phase the heap is only `a[0..end-1]`. Sifting with `n` would pull the big values already placed at the end back into the heap.
- **Building from the root down.** Sifting down only works when both subtrees are already heaps, so the build goes from the last parent back to the root. Done that way, building costs only `O(n)`.
- **Slow in practice.** Parents and children lie far apart in big arrays, which is hard on the memory cache, so heap sort is usually slower than quick sort despite the same `O(n log n)`.

## When to use it

Heap sort gives a guaranteed `O(n log n)` with no extra memory, which is why library sorts fall back to it when quick sort's recursion gets too deep. The heap itself is often more useful than the sort: it gives the biggest value in `O(1)` and takes a new value in `O(log n)`, which is exactly what a priority queue needs.

## Related topics

- [Selection Sort](learn:sorting/selection-sort): the same "take the extreme value" idea, with a slow scan.
- [Quick Sort](learn:sorting/quick-sort): usually faster, without the guarantee.
- Binary heap and priority queue (coming soon).
