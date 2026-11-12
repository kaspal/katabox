A neighbourhood library keeps a waiting line for every book that is out on loan. Process a list of events, where `ops[i]` is the event's name and `args[i]` its numbers:

- `"hold"`, args `[book, member]`: the member joins the **end** of the book's line.
- `"cancel"`, args `[book, member]`: the member leaves the book's line. If they are not in it, nothing happens.
- `"return"`, args `[book]`: a copy of the book comes back. It goes to the member at the **front** of the line, who leaves the line. **Report** that member, or `-1` if nobody is waiting (the copy goes back on the shelf).

A member may wait for several books at once, and may join a line again after leaving it (they go to the end). A member never holds the same book twice while already in its line.

Return the reports of the `"return"` events, in order.

### Example 1

```
ops = ["hold", "hold", "hold", "return", "cancel", "return", "return"], args = [[1, 10], [1, 20], [1, 30], [1], [1, 30], [1], [1]]
Output: [10, 20, -1]
```

The line for book 1 is 10, 20, 30. The first copy goes to 10; member 30 cancels; the next copy goes to 20; then nobody is waiting.

### Example 2

```
ops = ["return"], args = [[5]]
Output: [-1]
```

### Example 3

```
ops = ["hold", "hold", "cancel", "hold", "return", "return"], args = [[2, 7], [2, 8], [2, 7], [2, 7], [2], [2]]
Output: [8, 7]
```

Member 7 cancels and joins again, so they are now behind member 8.

### Constraints

- 1 ≤ ops.length ≤ 10⁵, args.length = ops.length
- Each `ops[i]` is `"hold"`, `"cancel"` or `"return"`, with the numbers described above
- 1 ≤ book, member ≤ 10⁹
