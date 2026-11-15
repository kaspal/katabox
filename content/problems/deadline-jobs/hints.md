Think about the best-paid job first: is there ever a reason to turn it down? Then the next best, and so on. The question is which day to give each job you keep.
---
Take the jobs from the highest fee to the lowest and put each one on the **latest** free day by its deadline, skipping it when every day up to its deadline is taken. Late days suit fewer jobs, so this keeps the early days open for jobs with tight deadlines. A deadline beyond `n` can be treated as `n`: no plan uses more than `n` days.
---
Searching back day by day for a free day is too slow when many jobs share a deadline. Keep `free[d]`, a link towards the latest free day ≤ `d`, with day 0 meaning "none left" (union-find over the days `0..n`). Follow the links (shortening them as you go) to find the day; when you take day `d`, set `free[d] = d − 1`.
