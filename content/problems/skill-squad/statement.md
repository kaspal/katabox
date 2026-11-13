A hackathon needs a team that covers `skills` different skills, numbered 0 to `skills − 1`. Each volunteer's skills are stored as one number: bit `s` of `people[i]` is 1 when volunteer `i` has skill `s`.

Return the size of the smallest team whose members together have every skill, or `-1` if even everyone together does not.

### Example 1

```
skills = 4, people = [3, 4, 12, 9]
Output: 2
```

3 is 0011 (skills 0 and 1) and 12 is 1100 (skills 2 and 3).

### Example 2

```
skills = 3, people = [1, 2]
Output: -1
```

Nobody has skill 2.

### Example 3

```
skills = 2, people = [3]
Output: 1
```

### Constraints

- 1 ≤ skills ≤ 14
- 1 ≤ people.length ≤ 60
- 0 ≤ people[i] < 2^skills
