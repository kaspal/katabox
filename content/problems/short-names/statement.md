A climbing club prints its members' names on small lockers, and the labels are tiny. For every name, the club wants the **shortest beginning** of it that no other member's name starts with, so that the label still points to exactly one member.

If even the whole name is the beginning of another member's name (like "cat" and "cattle"), the label is the whole name.

`names` lists the members (all names are different). Return the labels in the same order.

### Example 1

```
names = ["dog", "door", "cat", "cattle"]
Output: ["dog", "doo", "cat", "catt"]
```

"do" is also the start of "door", so "dog" needs all three letters. "cat" is the start of "cattle", so its label is the whole name; "cattle" is told apart by "catt".

### Example 2

```
names = ["zebra"]
Output: ["z"]
```

### Example 3

```
names = ["ab", "b", "abc"]
Output: ["ab", "b", "abc"]
```

### Constraints

- 1 ≤ names.length ≤ 2·10⁴
- 1 ≤ names[i].length ≤ 20, lowercase English letters
- All names are different
