## Approach: split, reverse the back half, interleave

1. **Middle:** move `slow` one step and `fast` two steps until `fast` reaches the end. With n nodes, the front half is the first ⌈n/2⌉ nodes.
2. **Cut and reverse:** cut after the front half and reverse the back half in place (three pointers).
3. **Interleave:** alternate one node from the front half and one from the reversed back half, relinking as you go.

### Walkthrough

`[3, 8, 1, 6, 9]`: front `[3, 8, 1]`, back `[6, 9]`, reversed `[9, 6]`. Interleaving gives `3, 9, 8, 6, 1`.

**Complexity:** O(n) time, O(1) extra space. Detaching the last node for every position walks the list each time, O(n²): about 5·10⁹ steps for 10⁵ passengers.
