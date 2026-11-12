A large cooperative has `n` members, numbered `0` to `n − 1`. `boss[i]` is the direct boss of member `i`, or `-1` for the president, who is above everyone.

When two members disagree, the dispute goes to their **closest common boss**: the lowest person in the hierarchy who is either one of the two or above both of them (directly or through other bosses). If one member is above the other, that member decides; a member is their own closest common boss.

`pairs[j] = [a, b]` describes dispute `j`. Return, for every dispute, who decides it.

### Example 1

```
boss = [-1, 0, 0, 1, 1, 2], pairs = [[3, 4], [3, 5], [4, 1], [2, 2]]
Output: [1, 0, 1, 2]
```

Members 3 and 4 both report to 1. Member 3 is under 1 and member 5 is under 2, so only the president is above both. Member 1 is above member 4, so 1 decides.

### Example 2

```
boss = [-1], pairs = [[0, 0]]
Output: [0]
```

### Example 3

```
boss = [2, 0, -1, 1], pairs = [[3, 0], [1, 3]]
Output: [0, 1]
```

The hierarchy is one line: 2 above 0 above 1 above 3.

### Constraints

- 1 ≤ n ≤ 5·10⁴
- Exactly one `boss[i]` is `-1`; every other is between `0` and `n − 1`
- Following `boss` from anyone always reaches the president (there are no loops)
- 1 ≤ pairs.length ≤ 5·10⁴, `pairs[j] = [a, b]` with 0 ≤ a, b < n
