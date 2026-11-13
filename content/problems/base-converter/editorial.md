## Approach: read into an integer, write it back out

**Reading:** each digit's value is its position in `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ`. Going left to right, `value = value · source + digit` (Horner's rule): "1011" in base 2 is ((1·2 + 0)·2 + 1)·2 + 1 = 11.

**Writing:** while `value > 0`, the next digit from the right is `value mod target`, then `value = ⌊value / target⌋`. Reverse the collected digits. If the value is 0, the answer is `"0"`.

The value is at most 10¹⁵, which every language here holds exactly in an ordinary number (JavaScript's limit for exact integers is about 9·10¹⁵).

### Walkthrough

"Z" in base 36 is 35. In base 3: 35 mod 3 = 2 (35 → 11), 11 mod 3 = 2 (→ 3), 3 mod 3 = 0 (→ 1), 1 mod 3 = 1 (→ 0). Reversed: "1022".

**Complexity:** O(digits) time.
