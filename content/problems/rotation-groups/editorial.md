## Approach: a canonical rotation per code

Two codes are the same bracelet when one is a rotation of the other. Rotation is an equivalence: every bracelet is a group of codes. To count groups without comparing pairs, give each group a **canonical** representative that every member can compute on its own: the alphabetically smallest rotation.

```
seen = empty set
for code in codes:
    smallest = min(code[i:] + code[:i] for i in 0 .. len(code) - 1)
    add smallest to seen
return size of seen
```

Codes of different lengths have rotations of different lengths, so they never collide.

### Walkthrough

| code | rotations | smallest |
| --- | --- | --- |
| lamp | lamp, ampl, mpla, plam | ampl |
| mpla | mpla, plam, lamp, ampl | ampl |
| palm | palm, almp, lmpa, mpal | almp |
| ampl | … | ampl |
| lmpa | lmpa, mpal, palm, almp | almp |

The set is {ampl, almp}: 2 bracelets.

**Complexity:** O(n · L²) time, with L ≤ 12, and O(n · L) space. Booth's algorithm finds the smallest rotation in O(L), which matters only for long codes. Comparing every pair of codes is O(n² · L²).
