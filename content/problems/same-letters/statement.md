A word game hands each player a bag of letter tiles. Two bags are **twins** when they hold exactly the same tiles: every letter appears the same number of times in both, whatever the order.

Given the tiles of two bags as strings `a` and `b`, return `true` if the bags are twins and `false` otherwise.

### Example 1

```
a = "pastel", b = "plates"
Output: true
```

### Example 2

```
a = "mood", b = "doom"
Output: true
```

### Example 3

```
a = "tool", b = "lots"
Output: false
```

`tool` has two `o` tiles and no `s`; `lots` has one `o` and an `s`.

### Constraints

- 1 ≤ a.length, b.length ≤ 10⁵
- a and b contain only lowercase English letters.
