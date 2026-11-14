## The idea

A queue is a line: **enqueue** adds a value at the back, **dequeue** takes the value at the front. The first value in is the first one out (FIFO). Building it on a plain array has a catch: taking the front value means shifting everything else left, which costs `O(n)` per dequeue.

A **ring buffer** (circular buffer) avoids shifting altogether. It uses a fixed array of `capacity` slots and two numbers: `head`, the slot of the front value, and `size`, how many values are queued. The back is not stored; it is computed as `tail = (head + size) mod capacity`. Values never move. Dequeue just moves `head` one slot on, and enqueue writes at `tail`.

### Wrapping around

The `mod capacity` is what makes the array a ring. When the tail runs past the last slot, it continues at slot 0, reusing the space that earlier dequeues freed. With capacity 4, `head = 2` and `size = 2`, the next value goes to `(2 + 2) mod 4 = 0`. `head` wraps the same way when it moves on.

### Full and empty

Because the capacity is fixed, enqueue on a full buffer (`size == capacity`) is refused. Dequeue on an empty one has nothing to give and returns `−1` here.

## Steps

1. Create `data` with `capacity` slots and set `head = 0`, `size = 0`.
2. enqueue `x`: if `size == capacity`, refuse it. Otherwise write `x` at `(head + size) mod capacity` and add 1 to `size`.
3. dequeue: if `size == 0`, return `−1`.
4. Otherwise read `data[head]`, set `head = (head + 1) mod capacity`, subtract 1 from `size`, and return the value.

## Cost

| Operation | Cost |
|---|---|
| enqueue | `O(1)` |
| dequeue | `O(1)` |
| Extra space | `O(capacity)`, fixed up front |

## Pitfalls

- **Keeping `head` and `tail` without `size`.** When `head == tail`, the buffer could be full or empty. Storing `size` (or leaving one slot always unused) tells the two apart.
- **Forgetting the wrap.** Writing at `head + size` without `mod capacity` runs off the end of the array.
- **Dequeuing from the front of a list.** `list.pop(0)` in Python or `shift()` in JavaScript moves every other value; a loop of them is `O(n²)`. Use a ring buffer or a deque.

## When to use it

Use a queue whenever work must be handled in arrival order: tasks waiting for a worker, requests in a rate limiter, or the frontier of a breadth-first search. A ring buffer suits fixed-size streams: audio and network buffers, the last `k` events, or a producer and consumer passing data with no allocation at all.

## Related topics

- [Stack](learn:linear/stack): the last-in, first-out counterpart.
- [Deque and monotonic queue](learn:linear/deque-monotonic-queue): a queue that also works at the back.
- [Dynamic array: insert and delete](learn:linear/dynamic-array-ops): why removing the front of an array is slow.
- [Sliding window](learn:searching/sliding-window): a window that enters at one end and leaves at the other.
- Breadth-first search (coming soon): explores a graph level by level with a queue.
