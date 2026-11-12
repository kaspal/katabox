## Approach: a min-heap of line fronts

The next customer of the merged line is the smallest ticket among the current fronts of the lines. A min-heap keeps those fronts:

1. Push `(ticket, line index)` for the front of every non-empty line.
2. Pop the smallest; append that node to the result and advance its line.
3. If the line still has a customer, push its new front.
4. Stop when the heap is empty.

The line index breaks ties, so the heap never compares nodes themselves. The result reuses the existing nodes, relinking them in order.

### Walkthrough

`lines = [[2, 5, 9], [1, 5], [3]]`: the heap starts with 2, 1, 3. Pop 1 (line 1, push 5), pop 2 (line 0, push 5), pop 3 (line 2 is empty now), pop 5, pop 5 (push 9), pop 9: `[1, 2, 3, 5, 5, 9]`.

**Complexity:** O(N log k) time for N customers in k lines, O(k) extra space for the heap. Scanning every line's front for each customer is O(N·k): with 10⁴ lines and 10⁵ customers that is 10⁹ comparisons.
