Read the whole input at once and split it into tokens: the line breaks stop mattering, and an empty line is simply no tokens.
---
Keep a position in the token list. Read `T`; then, for each day, read `n` and take the next `n` tokens as that day's prices.
---
For each day print `n`, the sum and the maximum of its prices, with 0 as the maximum when `n` is 0. Collect the lines and print them once at the end.
