The greedy rule "take the talk that ends first" no longer works: a short, dull talk can
block a long, valuable one. You need to compare choices, not commit to one.
---
Sort the talks by end time. For each talk, there are two options: skip it, or take it
together with the best programme among the talks that end by its start.
---
Let `best[i]` be the best total using the first `i` talks by end time. For talk `i`,
binary-search the ends for how many talks end at or before its start (`j`), then
`best[i + 1] = max(best[i], best[j] + score)`.
