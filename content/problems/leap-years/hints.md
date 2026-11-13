Check each year from `start` to `end` with a loop.
---
Write the rule as a condition: divisible by 4, and either not divisible by 100 or divisible by 400.
---
`y % 4 == 0 and (y % 100 != 0 or y % 400 == 0)`: count the years where it is true.
