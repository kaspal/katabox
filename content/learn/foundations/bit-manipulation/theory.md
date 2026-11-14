## The idea

A number is stored as a row of bits, each column a power of two: `44` is `00101100`, that is 32 + 8 + 4. Bitwise operators work on every column at once, in a single machine step. `bit_ops(x, y)` applies AND, OR, XOR and both shifts to two bytes, then counts the 1 bits of `x` with a classic trick: `x & (x - 1)` clears the lowest 1 bit, so the loop runs once per 1 bit rather than once per column.

## Steps

1. `x & y`: a 1 only in the columns where both have a 1.
2. `x | y`: a 1 in the columns where either has a 1.
3. `x ^ y`: a 1 in the columns where they differ.
4. `x << 1`: every bit moves one column left, which doubles `x`.
5. `x >> 1`: every bit moves one column right and the last one drops off: `x` halved, rounded down.
6. While `x != 0`: `x &= x - 1` and add one to `count`. Subtracting 1 flips the lowest 1 bit and the 0s below it, so the AND wipes out exactly that bit.

## Cost

| Operation | Cost |
|---|---|
| AND, OR, XOR, shifts | `O(1)`: one machine step on a whole word |
| Counting 1 bits | `O(number of 1 bits)`, at most the word size |
| Extra space | `O(1)` |

## Pitfalls

- **Operator precedence.** In JavaScript, `x & 1 === 0` means `x & (1 === 0)`, because comparison binds tighter than `&`. Always write `(x & 1) === 0`. Python happens to group it the other way, which makes the habit easy to lose.
- **JavaScript works on 32 bits.** Bitwise operators turn numbers into 32-bit signed integers: `1 << 31` is negative and big values lose their high bits. Use `>>>` for an unsigned right shift, or `BigInt` beyond 32 bits.
- **Negative numbers in Python.** Python integers have no fixed width, and a negative one behaves as if it had endless 1 bits on the left, so `while x != 0: x &= x - 1` never ends for `x < 0`. Mask first, for example `x & 0xFF`.

## When to use it

Use bits to store a set of small flags in one integer (bit `i` set means item `i` is in), to test or toggle a flag in `O(1)`, to walk every subset of up to about 20 items as the numbers `0 .. 2ⁿ − 1`, and for tricks such as XOR-ing a list to find the one value without a pair. `(x & (x - 1)) == 0` also tests whether a positive `x` is a power of two.

## Related topics

- [Big-O growth curves](learn:foundations/big-o-growth): the subset loop counts to `2ⁿ`, one bitmask per subset.
- [Prefix sums](learn:foundations/prefix-sums): another small trick that turns repeated work into `O(1)`.
- Bitmask dynamic programming (coming soon).
