Multiplying `exponent` times is hopeless for 10¹⁵. How can you get b⁸ from b⁴ with a single multiplication?
---
Squaring: b⁸ = (b⁴)². Any exponent is a sum of powers of two (its binary digits), e.g. 13 = 8 + 4 + 1, so b¹³ = b⁸ · b⁴ · b.
---
Loop over the bits of the exponent from the lowest: if the bit is 1, multiply the result by the current square; then square the current square. Take the remainder after every multiplication so the numbers stay small (below 1 000 003², about 10¹²).
