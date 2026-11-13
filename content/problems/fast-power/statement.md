A lock-maker's code generator multiplies a starting number by itself again and again: the code is `base` raised to the power `exponent`. The answer can be huge, so return it **modulo 1 000 003** (a prime): the remainder after dividing by 1 000 003. Any number to the power 0 is 1 (also 0⁰).

Return `base^exponent mod 1 000 003`.

### Example 1

```
base = 3, exponent = 13
Output: 594320
```

3¹³ = 1 594 323, and 1 594 323 − 1 000 003 = 594 320.

### Example 2

```
base = 10, exponent = 0
Output: 1
```

### Example 3

```
base = 1000003, exponent = 5
Output: 0
```

### Example 4

```
base = 7, exponent = 1000000000000000
Output: 942035
```

Multiplying 10¹⁵ times would take far too long.

### Constraints

- 0 ≤ base ≤ 10⁹
- 0 ≤ exponent ≤ 10¹⁵
