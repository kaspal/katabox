## The idea

A cache keeps a few answers close at hand so they need not be fetched or computed again. When it is full and a new entry arrives, something has to go. An **LRU** (least recently used) cache evicts the entry that has gone unused the longest, betting that what was used recently will be used again soon.

It needs two things at once:

- a **map** from key to value, so `get` finds an entry instantly;
- a **recency order**, from least recently used to most recently used, so the cache knows what to evict.

Every `get` that finds its key (a **hit**) moves that key to the most recent end. A `get` that misses returns `−1` and changes nothing. A `put` writes the value at the most recent end (moving the key there first if it was already cached), and if the cache now holds more than `capacity` keys, the one at the least recent end is evicted.

### Building the order

The samples lean on their languages: Python's `OrderedDict` keeps keys in order and can move one to the end (`move_to_end`) or drop the first (`popitem(last=False)`), and a JavaScript `Map` keeps insertion order, so deleting a key and setting it again moves it to the end. The animation draws that order as a doubly linked list, least recent on the left, with the map beside it.

That picture is how you build one by hand: a hash map from key to **node**, and a doubly linked list of nodes. The map finds the node in `O(1)`, and with `prev` and `next` links the node can be unlinked and moved to the end in `O(1)`.

## Steps

1. get `k`: if `k` is not in the map, return `−1` (a miss).
2. Otherwise move `k` to the most recent end and return its value (a hit).
3. put `k v`: if `k` is cached, move it to the most recent end. Store `v` for `k` at that end.
4. If there are now more than `capacity` keys, evict the one at the least recent end.

## Cost

| Operation | Cost |
|---|---|
| get | `O(1)` |
| put | `O(1)`, eviction included |
| Extra space | `O(capacity)`: one map entry and one list node per key |

## Pitfalls

- **Forgetting that `get` changes the order.** A read counts as a use. Without moving the key, the cache becomes first-in, first-out.
- **Evicting before writing.** Updating a key that is already cached never needs an eviction; check the size after the write, not before.
- **Removing from only one side.** On eviction, delete the key from the map and the node from the list.

## When to use it

Use an LRU cache when recent requests predict the next ones: web pages, database rows, results of slow function calls (Python's `functools.lru_cache` is exactly this).

## Related topics

- [Doubly linked list](learn:linear/doubly-linked-list): the recency order with `O(1)` moves.
- [Singly linked list](learn:linear/singly-linked-list): why one link per node is not enough here.
- Hash tables (coming soon): the map half of the cache.
