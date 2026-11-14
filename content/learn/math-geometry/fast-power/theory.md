## The idea

To compute `base^exp mod m`, the obvious loop multiplies by `base` `exp` times. With an exponent of a billion, that is a billion multiplications. **Square-and-multiply** needs only about two per **bit** of the exponent.

Write the exponent in binary. `13 = 1101₂ = 8 + 4 + 1`, so `x^13 = x^8 · x^4 · x^1`. The powers `x, x², x⁴, x⁸, …` come cheaply: each one is the **square** of the one before. So the loop walks the exponent's bits from the lowest: if the current bit is 1, it **multiplies** the result by the current power; then it squares the power and shifts the exponent right by one bit. When the exponent reaches 0, every 1 bit has contributed its power.

The second idea is to **reduce mod `m` after every multiplication**. `(x · y) mod m` equals `((x mod m) · (y mod m)) mod m`, so taking the remainder early never changes the answer, and every value stays below `m`. Without it, `7^1000` has 846 digits; with it, no product ever exceeds `(m - 1)²`.

The sample starts with `result = 1 mod m` (so a modulus of 1 gives 0) and `base mod m`. The output is `[result]`. The three numbers are typed as three fields: base, exponent and modulus.

### The default input

`7^1000 mod 13` (the "a big exponent" preset). In binary, `1000 = 1111101000₂`: ten bits, six of them 1. The current power `7^(2^i) mod 13` goes 7, 10, 9, 3, then alternates 9, 3, 9, 3, 9, 3, one value per bit.

The three lowest bits are 0, so the loop only squares. Bit 3 is the first 1: the result becomes 3. Bit 4 is 0; bits 5 to 9 are all 1, and the result alternates 9, 3, 9, 3, 9. The output is `[9]`, after 6 multiplications and 10 squarings, instead of 1000 multiplications.

## Steps

1. Set `result = 1 mod m` and `base = base mod m`.
2. While `exp > 0`: if its lowest bit is 1, set `result = result · base mod m`.
3. Set `base = base · base mod m` and shift `exp` right by one bit.
4. Return `result`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(log exp)`: one squaring per bit, and at most one multiplication |
| Extra space | `O(1)` |

The "a power of two" preset, `3^64 mod 101`, shows the cheapest shape: `64 = 1000000₂` has a single 1 bit, so 7 squarings and just 1 multiplication give `[88]`.

## Pitfalls

- **Reducing only at the end.** The intermediate power grows past any integer type almost at once.
- **Overflow of the product.** Two values below `m` multiply to almost `m²`; with a modulus near 10⁹, that needs 64-bit integers (and with one near 10¹⁸, 128-bit ones).
- **The modulus 1.** Everything is 0 mod 1; starting the result at a plain 1 would return 1.
- **A zero exponent.** The loop does not run and the result is `1 mod m`, also for `0⁰`.

## When to use it

Use it whenever a huge power is wanted modulo something: hashing, counting answers "modulo a prime", or checking a power cycle. The linked problem, Fast Power, asks exactly `base^exponent mod 1 000 003` with an exponent up to 10¹⁵. The same loop also gives modular inverses through Fermat's little theorem.

## Related topics

- [Bit manipulation basics](learn:foundations/bit-manipulation): reading the lowest bit and shifting right.
- [Modular inverse](learn:math-geometry/modular-inverse): with a prime modulus, the inverse is a fast power.
- [Recursion and the call stack](learn:foundations/recursion-call-stack): the same idea written recursively: for an even `e`, `x^e = (x^(e/2))²`.
- [Rolling hash](learn:hashing/rolling-hash): powers of a base modulo a number, one step at a time.
