## Approach: exponentiation by squaring

Write the exponent in binary. Keep `square = base mod p` (it will hold base¹, base², base⁴, base⁸, …) and `result = 1`. While `exponent > 0`:

- if its lowest bit is 1, `result = result · square mod p`;
- `square = square · square mod p`;
- `exponent = ⌊exponent / 2⌋`.

Every multiplication is followed by `mod p`, so both factors are below p = 1 000 003 and the product is below about 10¹², exact in every language here (even JavaScript's numbers, which are exact up to about 9·10¹⁵).

### Walkthrough

3¹³ with 13 = 1101 in binary: result picks up 3 (bit 1), skips 3² (bit 0), picks up 3⁴ = 81 and 3⁸ = 6561: 3 · 81 · 6561 = 1 594 323 → 594 320.

**Complexity:** O(log exponent) multiplications, about 50 for 10¹⁵. Multiplying `exponent` times is O(exponent).
