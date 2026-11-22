## Idea

With unlimited tokens, `best[a] = min(best[a − c] + 1)` over every value `c`. With one
copy of each token this becomes a 0/1 knapsack. Process the tokens one at a time, keeping
`best[a]` = the fewest of the tokens seen so far that make exactly `a` (`best[0] = 0`,
everything else unreachable). For a token `t`, update the amounts from `amount` down to
`t` with `best[a] = min(best[a], best[a − t] + 1)`. Because the loop goes downwards,
`best[a − t]` still describes groups without `t`, so no token is used twice. At the end,
`best[amount]` is the answer, or `-1` if it is still unreachable.

## Complexity

O(tokens.length · amount) time, at most 100 · 10⁴ = 10⁶ steps, and O(amount) extra
memory.
