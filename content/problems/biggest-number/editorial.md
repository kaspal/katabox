## Approach: sort with the "which pair reads bigger" rule

Write each card as text. For two cards `a` and `b`, put `a` first when `a + b > b + a` (string concatenation; the two results have the same length, so string comparison is numeric comparison).

This rule is a proper ordering: it is consistent across three or more cards (if `a` should precede `b` and `b` should precede `c`, then `a` should precede `c`), so a sort produces an arrangement where no swap of neighbours helps, and that is the biggest number.

Join the sorted texts. If the first character is `0`, every card was 0: return `"0"`.

### Walkthrough

Example 3: `4` vs `40`: "440" > "404", so 4 first. `4` vs `404`: "4404" > "4044", so 4 first. `404` vs `40`: "40440" > "40404", so 404 first. The order 4, 404, 40 gives "440440".

**Complexity:** O(n log n) comparisons of strings of at most 20 characters.
