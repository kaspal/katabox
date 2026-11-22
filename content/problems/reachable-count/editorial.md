## The bug

The breadth-first search marks a room as seen when it is pushed onto the queue, which is right, but the starting room is pushed without ever being marked:

```python
    seen = [False] * n
    queue = deque([start])
```

When a neighbor of `start` is processed, `seen[start]` is still `False`, so `start` is pushed and counted a second time. Any start room with at least one door is counted twice.

With `n = 2, doors = [[0, 1]], start = 0`: room 0 is popped (count 1) and pushes room 1; room 1 is popped (count 2) and pushes room 0 again; room 0 is popped (count 3). The starter returns 3, but only 2 rooms exist.

## The fix

Mark the start before the search begins:

```python
    seen = [False] * n
    seen[start] = True
    queue = deque([start])
```

The TypeScript starter returns `queue.length`, so the duplicate also shows up there; add `seen[start] = true;` before the loop.

**Complexity:** O(n + d) time and O(n + d) extra space, where d is the number of doors.
