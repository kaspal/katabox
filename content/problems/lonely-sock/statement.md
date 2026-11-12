The laundry room sorts socks by the number stitched inside each one. After the big Monday wash, the machine spits out a list of numbers, `ids`. Every number appears exactly **twice**, because socks come in pairs, except for one sock whose partner went missing.

Return the number of the lonely sock.

### Example 1

```
ids = [14, 3, 9, 3, 14]
Output: 9
```

### Example 2

```
ids = [72]
Output: 72
```

Only one sock came out, so it has no partner.

### Example 3

```
ids = [5, 8, 8, 1, 6, 1, 5]
Output: 6
```

### Constraints

- 1 ≤ ids.length ≤ 10⁵ (always odd)
- 1 ≤ ids[i] ≤ 10⁹
- Every value appears exactly twice, except one value that appears once.
