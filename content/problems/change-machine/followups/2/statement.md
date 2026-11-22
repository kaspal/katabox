This time the change machine is nearly empty: it holds just the tokens listed in
`tokens`, and **each token can be paid out at most once**. Several tokens may have the
same value; each of them is a separate token.

Return the fewest tokens from the machine that add up to exactly `amount`, or `-1` if no
group of them does. Paying `0` takes no tokens.

### Example 1

```
tokens = [5, 1, 5, 2, 10], amount = 11
Output: 2
```

10 + 1.

### Example 2

```
tokens = [3, 2, 5, 2, 8], amount = 15
Output: 3
```

8 + 5 + 2; no two tokens add up to 15.

### Example 3

```
tokens = [1, 2, 2, 6, 3], amount = 11
Output: 3
```

### Constraints

- 1 ≤ tokens.length ≤ 100
- 1 ≤ tokens[i] ≤ 10⁴ (values may repeat)
- 0 ≤ amount ≤ 10⁴
