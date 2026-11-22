Try the smallest query there is: a single day, `[l, l]`. What should it return, and what does the code return?
---
Building the prefix list looks fine. Look at how a query turns `l` and `r` into two positions of `prefix`. What does `prefix[i]` actually hold?
---
`prefix[i]` is the total of the first `i` days, that is, days `0` to `i - 1`. So `prefix[r] - prefix[l]` covers days `l` to `r - 1` and misses day `r`. Use `prefix[r + 1] - prefix[l]`.
