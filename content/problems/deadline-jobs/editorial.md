## Approach: best fee first, on the latest free day (union-find over days)

**The greedy.** Sort the jobs by fee, highest first. Give each one the latest free day up to its deadline; if every day up to its deadline is taken, turn it down. Deadlines above `n` are cut to `n`: at most `n` jobs are done, so any plan fits in days `1..n`.

Example 1, in fee order: 50 (due day 2) takes day 2; 40 (due day 2) takes day 1; 30 (due day 1) finds days 1 and 2 taken and is turned down; 20 (due day 3) takes day 3; 10 is turned down. Total 110.

**Why it is correct.** Use this test: a set of jobs can all be done exactly when, for every day `d`, at most `d` of them are due by day `d` (then doing them in deadline order meets every deadline).

- *The day search answers "does this job still fit?"* If every day up to the job's deadline is taken, let `m` be the end of the run of taken days starting at day 1. Every job on days `1..m` is due by day `m`: one due later would have been put on the free day `m + 1` or later. So `m` taken jobs and the new one are all due by day `m`, and the test fails. Otherwise there is a free day and the job fits.
- *Keeping a job that fits never hurts.* Suppose the jobs kept so far, `G`, are part of some best plan `B`, and the next job `j` fits with `G` but is not in `B`. If `B + j` passes the test, it pays more than `B`: impossible. Otherwise take the first day `d` where `B + j` has `d + 1` jobs due. `G + j` passes the test, so some job `k` due by day `d` is in `B` but not in `G`. Jobs earlier in fee order that were turned down did not fit with part of `G`, so they are not in `B` either: `k` comes later in fee order, and its fee is at most `j`'s. Swapping `k` for `j` clears every overloaded day (they are all `d` or later, and `k` is due by `d`) and pays no less. So some best plan contains `G + j`.
- *Turning down a job that does not fit is forced:* no plan containing `G` can include it.

By induction the kept jobs form a best plan.

**Finding the day fast.** Searching back one day at a time is the slow part: with 10⁵ jobs all due by day 5·10⁴, half of them scan almost every day, about 3.7·10⁹ steps. Instead keep `free[d]`, which points towards the latest free day ≤ `d`, with day 0 standing for "no free day left". Start with `free[d] = d`. To place a job, follow the pointers from `min(deadline, n)` until a day points to itself, halving the path as you go (`free[d] = free[free[d]]`). Taking day `d` sets `free[d] = d − 1`, so later searches jump straight past it.

**Complexity:** O(n log n) for the sort, plus nearly O(1) per job for the union-find: O(n log n) time, O(n) memory.

**Practical notes:** read all tokens at once. The total can reach 10⁵ · 10⁹ = 10¹⁴: use `u64` or `i64` in Rust; in TypeScript it stays below 2⁵³, so `Number` is exact, and PHP integers are 64-bit. Write the find as a loop, not recursion (a chain of 10⁵ days would overflow Python's recursion limit). Ties in fee can be taken in any order.
