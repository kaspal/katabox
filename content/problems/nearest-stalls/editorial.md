## Approach: rank by (squared distance, index) with a heap

Use the key `(x² + y², i)` for stall `i`: squared distances compare the same way as distances, stay exact in integers (at most 2·10⁸), and the index settles ties.

Keep a **max-heap** of at most k keys. For each stall, push its key; if the heap now holds more than k, pop the largest. At the end the heap holds the k smallest keys; sort them and output their indices.

Sorting all n keys and taking the first k is also correct, in O(n log n).

**Complexity:** O(n log k) time, O(k) extra space. Selecting the minimum k times with a full scan is O(n·k): 5·10⁹ steps for n = 10⁵ and k = n / 2.
