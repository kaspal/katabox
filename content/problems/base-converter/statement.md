A puzzle box has dials that show numbers in different bases. Digits go `0`-`9` and then `A`-`Z` for the values 10 to 35, so base 36 is the largest.

Given `number` written in base `source`, return the same number written in base `target`, with uppercase letters and no leading zeros (zero itself is `"0"`).

### Example 1

```
number = "255", source = 10, target = 16
Output: "FF"
```

### Example 2

```
number = "1011", source = 2, target = 10
Output: "11"
```

### Example 3

```
number = "0", source = 7, target = 2
Output: "0"
```

### Example 4

```
number = "Z", source = 36, target = 3
Output: "1022"
```

Z is 35 = 1·27 + 0·9 + 2·3 + 2.

### Constraints

- 2 ≤ source, target ≤ 36
- `number` uses only digits valid in base `source` (uppercase letters), has no leading zeros unless it is `"0"`, and its value is at most 10¹⁵.
