A community garden plants a straight bed `length` centimetres long, one plant after another. There are several **kinds** of plant: a plant of kind `i` takes exactly `widths[i]` centimetres. Two kinds may need the same width and are still different kinds.

The bed must be filled **exactly**, with no gap at the end, and two plants of the **same kind** may not stand next to each other (pests spread too easily). Kinds can be used any number of times otherwise.

How many different plantings are there? Two plantings differ if the sequences of kinds differ. The number can be huge: return it modulo 1 000 000 007.

### Example 1

```
length = 4, widths = [1, 2]
Output: 1
```

With kinds A (1 cm) and B (2 cm), only A B A fills 4 cm without two equal neighbours: B B and A A A A repeat a kind, and A A B or B A A put two A together.

### Example 2

```
length = 3, widths = [1, 1, 1]
Output: 12
```

Three kinds of width 1: 3 choices for the first plant, then 2 for each of the next two.

### Example 3

```
length = 1, widths = [2]
Output: 0
```

### Constraints

- 1 ≤ length ≤ 10⁴
- 1 ≤ widths.length ≤ 50
- 1 ≤ widths[i] ≤ 100
