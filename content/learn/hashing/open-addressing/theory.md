## The idea

**Open addressing** keeps a hash table in one plain array of slots, one key per slot, with no lists at all. Each key has a **home slot**, here `key mod 8`. If the home slot is taken by another key, the table **probes**: with **linear probing** it tries the next slot, then the next, wrapping around at the end, until it finds the key or an empty slot.

insert walks the probe path and puts the key in the first empty slot. find walks the same path: meeting the key means found (1); meeting an empty slot means the key cannot be further on, so it is not there (0).

### Tombstones

Deleting is the tricky part. If delete simply emptied the slot, a later find for a key stored *past* it would stop at the gap and wrongly answer 0. So delete leaves a **tombstone** (✕): a marker meaning "something was here, keep probing".

The sample keeps things minimal: insert never reuses tombstones, and it allows at most 7 inserts so at least one slot always stays empty and every probe stops. Real tables do reuse tombstones for new keys, and they **resize** (allocate a bigger array and rehash every live key) once the table gets too full.

### Clustering

The default input inserts 3, 11, 19 and 4. The first three all have home slot 3, so they fill slots 3, 4 and 5. Key 4 has home slot 4, which is taken, so it lands in slot 6. Occupied runs like this are **clusters**: any key hashing into a cluster joins it and makes it longer. find 27 (home 3) has to walk all four keys before reaching empty slot 7.

## Steps

1. Start with 8 empty slots.
2. Compute the home slot, `i = key mod 8`.
3. While slot `i` is neither empty nor the key, move on: `i = (i + 1) mod 8`.
4. insert: put the key in slot `i`. find: output 1 if slot `i` holds the key, else 0.
5. delete: if slot `i` holds the key, replace it with a tombstone.

## Cost

| Operation | Cost |
|---|---|
| insert, find, delete, average | `O(1)` while the table is not too full |
| insert, find, delete, worst | `O(m)`: one long cluster |
| Extra space | `O(m)` for `m` slots |

## Pitfalls

- **Emptying a slot on delete.** It breaks every probe path that ran through it. Leave a tombstone.
- **Letting the table fill up.** With no empty slot, a search for a missing key loops forever. Resize long before that.
- **Ignoring clusters.** Linear probing slows sharply as the table fills; quadratic steps or a second hash spread keys out.

## When to use it

Open addressing stores keys right in the array, so probes read neighbouring memory and are cache friendly. Python's `dict` uses it. Prefer chaining when deletes are frequent or the table must run nearly full.

## Related topics

- [Hash table: separate chaining](learn:hashing/separate-chaining): colliding keys share a list instead.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): the same wrap-around indexing with `mod`.
- [Bloom filter](learn:hashing/bloom-filter): hashing that answers only "maybe" or "no".
