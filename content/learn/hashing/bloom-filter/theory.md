## The idea

A **Bloom filter** answers one question, "have I seen this key?", using far less memory than storing the keys. It is a row of `m` bits, all 0 at the start, plus `k` hash functions that each map a key to one bit. This one has 16 bits and three hashes of the form `(a · key + b) mod 16`, with `(a, b)` equal to `(1, 0)`, `(7, 3)` and `(11, 5)`.

- **add** a key: compute its three positions and set those bits to 1.
- **check** a key: look at its three bits. If any of them is 0, the key was **definitely not** added, because adding it would have set that bit. If all three are 1, the answer is only **maybe**: other keys may have set those bits.

### False positives, never false negatives

The filter never stores keys, only bits that keys share. In the default input, add 7 sets bits 7, 4 and 2, and add 12 sets bits 12, 7 and 9 (bit 7 was already set). Then check 23 computes bits 7, 4 and 2, exactly the bits of 7, and says "maybe" although 23 was never added: a **false positive**. check 5 stops at once, because its first bit, 5, is clear: definitely not.

A "no" is always right, so there are no false negatives. The price is that a filter cannot **delete**: clearing a bit could erase part of another key and turn a correct "maybe" into a wrong "no".

The fuller the filter, the more often an unrelated key finds all its bits set by chance. About 10 bits per key give roughly 1% false positives.

## Steps

1. Start with 16 bits, all 0.
2. add `key`: for each of the three hashes, set bit `(a · key + b) mod 16` to 1.
3. check `key`: for each hash in turn, read the bit. If it is 0, output 0 (definitely not) and stop.
4. If all three bits are 1, output 1 (maybe).

## Cost

| Operation | Cost |
|---|---|
| add | `O(k)` for `k` hashes, whatever the number of keys |
| check | `O(k)`, often less: it stops at the first clear bit |
| Extra space | `O(m)` bits, far smaller than the keys themselves |

## Pitfalls

- **Treating "maybe" as "yes".** A maybe must be confirmed against the real data when the answer matters.
- **Removing keys by clearing bits.** It creates false negatives. A counting Bloom filter, with small counters instead of bits, is needed for deletes.
- **Sizing it too small.** Once most bits are 1, nearly every check says maybe and the filter stops filtering.

## When to use it

Put a Bloom filter in front of an expensive lookup that usually misses: a database checks it before reading a disk block, a browser before asking a server whether a link is unsafe, a crawler before revisiting a page. Every "definitely not" saves the slow lookup.

## Related topics

- [Hash table: separate chaining](learn:hashing/separate-chaining): exact answers, at the cost of storing every key.
- [Bit manipulation basics](learn:foundations/bit-manipulation): setting and testing single bits.
- [Hash table: open addressing](learn:hashing/open-addressing): another fixed-size array driven by hashes.
