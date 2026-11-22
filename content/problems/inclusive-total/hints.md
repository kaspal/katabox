Try the smallest possible range, where `a` and `b` are the same index. What should the answer be, and what does the code return?
---
Look at the loop bounds. Which indices does the loop actually visit?
---
`range(a, b)` stops before `b`, so `nums[b]` is never added. Loop up to `b + 1` instead (in TypeScript, use `i <= b`).
