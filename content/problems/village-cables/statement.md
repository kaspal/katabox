A valley has `n` villages, numbered `0` to `n − 1`, and wants a fibre network that lets every village reach every other one, directly or through other villages. Surveyors listed the cables that could be laid: `cables[i] = [a, b, cost]` is a cable between villages `a` and `b` costing `cost` coins.

Return the **lowest total cost** of a set of cables that connects all the villages, or `-1` if even laying every listed cable leaves some village cut off.

### Example 1

```
n = 4, cables = [[0, 1, 7], [1, 2, 3], [0, 2, 5], [2, 3, 4], [1, 3, 9]]
Output: 12
```

Lay 1–2 (3), 2–3 (4) and 0–2 (5). Every village is reached for 12 coins.

### Example 2

```
n = 3, cables = [[0, 1, 2]]
Output: -1
```

Village 2 has no cable at all.

### Example 3

```
n = 1, cables = []
Output: 0
```

### Constraints

- 1 ≤ n ≤ 2·10⁴
- 0 ≤ cables.length ≤ 4·10⁴
- `cables[i] = [a, b, cost]` with 0 ≤ a, b < n, a ≠ b and 1 ≤ cost ≤ 10⁶; two cables may join the same villages
