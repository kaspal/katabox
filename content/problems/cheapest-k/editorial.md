## The bug

The idea is right: keep at most `k` prices in a heap and, whenever there are `k + 1`, drop the one that cannot be among the `k` cheapest, which is the most expensive. But `heapq` is a min-heap, so these lines drop the cheapest instead:

```python
        heapq.heappush(heap, p)
        if len(heap) > k:
            heapq.heappop(heap)
```

The heap ends up with the `k` largest prices, and the starter returns their sum. It is only right when `k` equals the length of the list, since then nothing is ever popped.

With `prices = [1, 5], k = 1`: after pushing 5 the heap holds 1 and 5, `heappop` removes 1, and the starter returns 5. The answer is 1.

## The fix

Turn it into a max-heap by storing negated prices, so the root is the most expensive price kept:

```python
    for p in prices:
        heapq.heappush(heap, -p)
        if len(heap) > k:
            heapq.heappop(heap)
    return -sum(heap)
```

In TypeScript, the `Heap` class takes the ordering as an argument, so create it with `new Heap((a, b) => a > b)`.

**Complexity:** O(n log k) time, O(k) extra space.
