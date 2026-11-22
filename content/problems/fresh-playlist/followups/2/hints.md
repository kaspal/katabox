For one request, a greedy cut works: make each block as long as it can be, starting
where the previous one ended. Why is that never worse? And with 10⁵ requests, the greedy
walk itself (up to 10⁵ blocks per request) is still too slow.
---
From play `i`, the longest fresh block ends just before `next[i]`, and `next[i]` never
decreases as `i` grows; one pass of the base problem's window computes it for every `i`.
A request is then "start at `a`, follow `next` while it stays at most `b`, count the
steps".
---
Following `next` many times is what binary lifting is for: `jump[k][i]` is where `2^k`
blocks starting at `i` lead (`jump[0] = next`, `jump[k][i] = jump[k−1][jump[k−1][i]]`). For
a request, take the largest jumps that stay at most `b`, adding `2^k` blocks each time;
the answer is one more than the blocks jumped over.
