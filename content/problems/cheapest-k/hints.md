Try the smallest case where the heap has to drop something: `prices = [1, 5], k = 1`. Which price is left at the end?
---
Each time the heap grows past `k`, one price is popped. Which price does `heappop` remove, and is that the one you want to throw away?
---
`heapq` is a min-heap, so popping removes the cheapest price and the heap ends up holding the `k` most expensive ones. To drop the most expensive price instead, use a max-heap: push `-p` and return `-sum(heap)`. In TypeScript, build the heap with `(a, b) => a > b` so the largest value comes out first.
