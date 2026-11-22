The base problem's sliding window relied on costs never being negative: growing a stretch
never made it cheaper. With refunds, a stretch that is too expensive can become affordable
again further on. Can prefix sums help?
---
With `prefix[j]` the cost of the first `j` stages, the stretch `i..j−1` fits when
`prefix[i] ≥ prefix[j] − budget`. For each end `j`, you want the **earliest** such `i`.
Which starts can ever be the earliest one?
---
A start is only worth keeping if its prefix is higher than every earlier prefix: an
earlier start that is at least as high is always better. Those record highs form an
increasing list, so for each `j`, binary-search it for the first value ≥ `prefix[j] −
budget`, then add `prefix[j]` to the list if it is a new high.
