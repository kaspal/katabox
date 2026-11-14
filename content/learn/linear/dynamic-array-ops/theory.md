## The idea

A dynamic array keeps its values side by side in one block of storage, packed from position 0. `size` says how many slots are in use; the rest are spare. To **insert** at position `i`, every value from `i` to the end moves one place right to open a gap. To **remove** position `i`, every value after it moves one place left to close the gap. Appending is just an insert at position `size`: nothing sits after it, so nothing moves.

When every slot is in use, an insert first **doubles** the storage: a new block twice as big, with the old values copied over. [Amortised cost: the dynamic array](learn:foundations/dynamic-array) shows why that copying averages out to `O(1)` per append.

### Why shift from the end

To open a gap, start with the **last** value: it moves into the spare slot, the one before it moves into the place it left, and so on down to `i`. Starting at `i` would overwrite `a[i + 1]` before it had moved. Closing a gap is the mirror image: shift left, starting just after the removed slot.

## Steps

1. Insert `value` at `i`: if `size` equals the capacity, double the storage.
2. For `k` from `size` down to `i + 1`, copy `data[k - 1]` into `data[k]`.
3. Write `value` into `data[i]` and add 1 to `size`.
4. Remove at `i`: for `k` from `i` to `size - 2`, copy `data[k + 1]` into `data[k]`.
5. Subtract 1 from `size`; the last slot is spare again.

## Cost

| Operation | Cost |
|---|---|
| Read or write `a[i]` | `O(1)` |
| Append | `O(1)` amortised; `O(n)` on the step that doubles |
| Insert at `i` | `O(n − i)`: one shift per later value |
| Remove at `i` | `O(n − i)` |
| Extra space | Up to `n` spare slots right after a doubling |

## Pitfalls

- **Shifting in the wrong direction.** Opening a gap from the left copies `a[i]` over `a[i + 1]`, then that copy over `a[i + 2]`, and the whole tail turns into one repeated value.
- **Forgetting `size`.** Spare slots still hold old values. Read only `data[0..size - 1]`, and update `size` on every insert and remove.
- **Front-heavy loops.** Inserting or removing at position 0 over and over makes a loop `O(n²)`. If you need a fast front, use a deque or a ring buffer.

## When to use it

A dynamic array is the default list in most languages: compact, cache friendly and with instant indexing. It fits best when most changes happen at the end. When inserts and removes in the middle dominate, a linked list or a balanced tree may fit better.

## Related topics

- [Amortised cost: the dynamic array](learn:foundations/dynamic-array): why doubling keeps appends cheap.
- [Insertion Sort](learn:sorting/insertion-sort): the same shift right to open a gap.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): a fixed array that never shifts.
- [Singly linked list](learn:linear/singly-linked-list): inserts without shifting, but no instant `a[i]`.
