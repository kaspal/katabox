A calendar printer needs to know how many of the years from `start` to `end` (both included) have a 29th of February. A year is a **leap year** when:

- it is divisible by 4, **and**
- it is not divisible by 100, **unless** it is also divisible by 400.

So 2024 and 2000 are leap years, but 1900 and 2023 are not.

Return the number of leap years from `start` to `end`.

### Example 1

```
start = 1996, end = 2004
Output: 3
```

1996, 2000 and 2004.

### Example 2

```
start = 1900, end = 1900
Output: 0
```

1900 is divisible by 100 but not by 400.

### Example 3

```
start = 2000, end = 2000
Output: 1
```

### Constraints

- 1 ≤ start ≤ end ≤ 9999
