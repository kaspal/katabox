## Approach: a 2-D difference array

Use a table `d` with one extra row and column. For each stamp, write four marks:

- `+1` at `(top, left)`, where the rectangle starts;
- `−1` at `(top, right + 1)` and at `(bottom + 1, left)`, just past its right and bottom edges;
- `+1` at `(bottom + 1, right + 1)`, because that corner was cancelled twice.

Now take a running sum along each row, then a running sum down each column. Each cell ends up with the sum of all marks above and to its left, which is exactly the number of stamps covering it: a stamp's four marks cancel out everywhere outside its rectangle.

Finally scan the `rows × cols` cells for the maximum count and how many cells have it.

### Walkthrough

Example 3 is one row: marks `+1` at 0, `−1` at 2, `+1` at 3, `−1` at 5. The running sum is `1 1 0 1 1`: the maximum 1 appears 4 times.

**Complexity:** O(stamps + rows · cols) time and O(rows · cols) space. Inking every cell of every stamp is O(stamps · rows · cols), about 10¹⁰ steps for 6·10⁴ stamps covering most of a 500 × 500 sheet.
