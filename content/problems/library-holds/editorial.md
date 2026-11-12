## Approach: queues with lazy cancellation and tickets

Keep, for every book, a **queue** of waiting entries, and a hash map `waiting[(book, member)]` with the member's current **ticket**, a number that grows with every hold.

- `hold(book, member)`: take a new ticket `t`; set `waiting[(book, member)] = t`; push `(member, t)` at the end of the book's queue.
- `cancel(book, member)`: delete `waiting[(book, member)]` if present. The queue is left alone.
- `return(book)`: pop entries from the front of the book's queue until one is still valid, that is `waiting[(book, member)] == t`. That member gets the copy: delete their map entry and report them. If the queue runs out, report `-1`.

Entries of cancelled members are thrown away the first time a return reaches them (**lazy deletion**). The tickets matter when a member cancels and holds again: the old entry `(member, t_old)` is still in the queue, ahead of the new one, but the map now holds `t_new`, so the old entry is skipped and the member keeps their new place at the end.

### Walkthrough

Example 3: holds give 7 ticket 1 and 8 ticket 2. Member 7 cancels and holds again with ticket 3, so the queue is (7, 1), (8, 2), (7, 3). The first return skips (7, 1) (stale) and gives the copy to 8; the second gives it to 7 via (7, 3). Reports: 8, 7.

**Complexity:** every entry is pushed once and popped at most once, so O(ops) expected time overall, and O(ops) memory. Removing cancelled members from a list immediately costs O(line length) per cancel, O(ops²) in the worst case.
