Build the brain of a pocket calculator. `expression` is an arithmetic expression made of:

- non-negative whole numbers (no leading zeros, except the number `0` itself);
- the operators `+`, `-` and `*`;
- parentheses `(` and `)`;
- spaces, which mean nothing.

The usual rules apply: `*` binds tighter than `+` and `-`, operators of the same kind are applied from left to right, and parentheses group. A `-` can also be a **unary minus** at the start of the expression, after `(` or right after another operator (`2 * -3`, `-(1 + 2)`, `5 - -1`).

Return the value. Every intermediate result fits comfortably: its absolute value stays below 2⁵³.

### Example 1

```
expression = "2 + 3 * 4"
Output: 14
```

### Example 2

```
expression = "(2 + 3) * 4"
Output: 20
```

### Example 3

```
expression = "-(7 - 10) * -2"
Output: -6
```

The parentheses give −3, the unary minus makes it 3, and 3 · −2 = −6.

### Example 4

```
expression = "10 - 2 - 3"
Output: 5
```

Left to right: (10 − 2) − 3.

### Constraints

- 1 ≤ expression.length ≤ 10⁴
- The expression is valid as described; parentheses and unary minus signs are nested at most 100 deep
- Every intermediate value has absolute value < 2⁵³
