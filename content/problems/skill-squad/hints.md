Trying every team means up to 2⁶⁰ teams. But what really matters about a partial team is only **which skills** it already covers. How many different "covered" sets are there?
---
At most 2^skills = 16 384. Think of each covered set (a mask) as a node; adding volunteer `p` moves from `mask` to `mask | people[p]`.
---
The fewest volunteers to reach the full mask is a shortest path in this graph: a BFS from mask 0, where each step adds one volunteer. Remove duplicate volunteers first; the answer is the distance to `2^skills − 1`, or −1 if it is never reached.
