## The idea

A hash table stores keys so that finding one does not mean looking at all of them. A **hash function** turns each key into a bucket number, and the key is kept in that bucket. Here the table has 7 buckets and the hash is simply `key mod 7`: key 31 goes to bucket 3, key 22 to bucket 1.

Two different keys can land in the same bucket; that is a **collision**. With **separate chaining**, each bucket is a small list (a chain), and colliding keys simply share it. To insert a key, the table hashes it, scans that one chain for a duplicate, and appends the key only if it is new.

### Load factor

How long do the chains get? With `n` keys spread over `m` buckets, the average chain holds `n / m` keys, the **load factor**. The default input puts 9 keys into 7 buckets, a load factor of about 1.3, yet bucket 3 ends up with three keys (10, 31 and 17) while buckets 2 and 4 stay empty. A good hash spreads keys evenly; a bad one, or unlucky keys, piles them into a few chains. In the worst case every key shares one bucket and the table is just a slow list.

Real tables keep the load factor small by **resizing**: past a limit (often around 1), they double the buckets and rehash every key, an `O(n)` step rare enough to cost `O(1)` amortised per insert.

## Steps

1. Start with 7 empty buckets.
2. For each key, compute its bucket, `b = key mod 7`.
3. Scan bucket `b` for the key. If it is already there, skip it.
4. Otherwise append the key to the end of bucket `b`.
5. The output lists each bucket in order: its length, then its keys.

## Cost

| Operation | Cost |
|---|---|
| Insert or find, average | `O(1 + n/m)`: one hash, then a short chain |
| Insert or find, worst | `O(n)`: every key in one chain |
| Delete | Same as find, then remove from the chain |
| Extra space | `O(n + m)`: the buckets plus one list entry per key |

## Pitfalls

- **Forgetting the duplicate scan.** Appending without looking first stores the same key twice, and a later delete may remove only one copy.
- **A hash that ignores part of the key.** Hashing a string by its first letter alone sends every word starting with "s" to one bucket. Use all of the key.
- **Never resizing.** With a fixed number of buckets, chains grow linearly with the number of keys and lookups slow to `O(n)`.

## When to use it

Use a hash table whenever you need "is this key here?" or "what is stored for this key?" in constant time on average: counting words, grouping anagrams, remembering seen values. Python's `dict` and `set` and JavaScript's `Map` and `Set` already do this for you.

## Related topics

- [Hash table: open addressing](learn:hashing/open-addressing): no lists, keys live in the slots themselves.
- [Singly linked list](learn:linear/singly-linked-list): the classic way to build each chain.
- [Amortised cost: the dynamic array](learn:foundations/dynamic-array): why doubling on resize stays cheap.
- [LRU cache](learn:linear/lru-cache): a hash map paired with a recency list.
