Run the code by hand on the smallest map with a door: `n = 2, doors = [[0, 1]], start = 0`. Which rooms go through the queue, and how many times each?
---
Every room is supposed to enter the queue exactly once. Look at which rooms get marked in `seen`, and when.
---
`start` goes into the queue without being marked as seen, so its first neighbor puts it in the queue again and it is counted twice. Set `seen[start] = True` (`seen[start] = true` in TypeScript) before the loop.
