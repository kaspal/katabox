## Approach: a sweep line with a max-heap

The outline is constant between consecutive **critical positions**, the lefts and rights of all buildings. Sort the buildings by `left` and the distinct critical positions ascending, and sweep:

- push every building whose `left ≤ x` onto a max-heap keyed by height, remembering its `right`;
- while the heap's top has `right ≤ x`, it no longer covers `x`: pop it. Finished buildings that are not on top stay in the heap; they are harmless until they reach the top, and then they are popped (lazy removal);
- the outline at `x` is the top's height, or 0 with an empty heap. If it differs from the previous height, record `[x, height]`.

### Walkthrough

Example 1: at 1, building 3 → [1, 3]; at 2, the 5 joins → [2, 5]; at 4, the 3 ends but is not on top, still 5; at 6, the 5 ends, then the 3 (finished) is popped → [6, 0]; at 8 → [8, 2]; at 9 → [9, 0].

**Complexity:** O(n log n) time, O(n) space. Computing the outline at each critical position by scanning every building is O(n²).
