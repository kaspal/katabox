At a fairground stall, you get a handful of number cards and lay **all** of them side by side in a row to form one big number; the stall pays out more for bigger numbers.

Given the numbers on the cards, return the biggest number you can form, as a string. It has no leading zeros, except that a number made only of zeros is written `"0"`.

### Example 1

```
cards = [7, 76, 760, 8]
Output: "8776760"
```

8, then 7, then 76, then 760.

### Example 2

```
cards = [0, 0]
Output: "0"
```

### Example 3

```
cards = [40, 4, 404]
Output: "440440"
```

### Constraints

- 1 ≤ cards.length ≤ 1000
- 0 ≤ cards[i] ≤ 10⁹
