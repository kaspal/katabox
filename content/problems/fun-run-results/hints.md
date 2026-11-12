The board order compares runners by two things: first the time, then, only when the times are equal, the name.
---
Sort the runner indices (or pairs `(time, name)`) instead of sorting the two lists separately, so each name stays with its own time.
---
Most languages sort tuples or pairs lexicographically: sorting by the key `(time, name)` does exactly what the board needs. Otherwise write a comparator: compare times, and compare names only on a tie.
