## The idea

Pay an exact **amount** with the **fewest coins**, using an unlimited supply of each coin value. The obvious plan is greedy: take the largest coin that fits, repeat. It works for many real currencies, but not for every set of coins.

Dynamic programming asks a smaller question for every total from 0 up to the amount: what is the fewest number of coins that make **exactly this total**? Call it `fewest[t]`. Whatever the best way to pay `t` is, it has some **last coin** `c`, and what comes before it is a best way to pay `t - c`. So:

`fewest[t] = min over coins c ≤ t of fewest[t - c] + 1`

`fewest[0] = 0`: no coins pay nothing. Every other cell starts at 999, the sample's stand-in for "cannot be made yet" (larger than any real count here). The sample tries the coins in the order given and **improves** a cell only when `fewest[t - c] + 1` is strictly smaller. If `fewest[amount]` is still 999 at the end, no combination works and the output is `[-1]`; otherwise it is `[fewest[amount]]`.

### The default input

Amount 12, coins 1, 6 and 9 (the "greedy fails" preset). Greedy takes 9, leaving 3, which only 1s can pay: 9 + 1 + 1 + 1, **four coins**.

The table: totals 1 to 5 can only use 1s, so they get 1 to 5. Total 6 first gets 6 from coin 1, then coin 6 improves it to 1. Totals 7 and 8 get 2 and 3. Total 9 drops to 1 with coin 9. Totals 10 and 11 get 2 and 3. For 12, coin 1 offers `fewest[11] + 1 = 4`, coin 6 offers `fewest[6] + 1 = 2`, and coin 9 offers `fewest[3] + 1 = 4`. The table is `[0, 1, 2, 3, 4, 5, 1, 2, 3, 1, 2, 3, 2]`.

The output is `[2]`: 6 + 6. Greedy committed to the 9 and never looked back; the table compares every possible last coin.

## Steps

1. Set `fewest[0] = 0` and every other total to 999.
2. For each total `t` from 1 to the amount, and each coin `c ≤ t`: if `fewest[t - c] + 1 < fewest[t]`, set `fewest[t]` to it.
3. Return `-1` if `fewest[amount]` is still 999, else `fewest[amount]`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(amount · coins)`: each total tries each coin once |
| Extra space | `O(amount)` for the table |

## Pitfalls

- **Trusting greedy.** It is only safe for coin sets known to be greedy-friendly; one counterexample like 1, 6, 9 is enough to break it.
- **Adding 1 to "impossible".** With a sentinel like 999, `999 + 1` stays too large to win, so it is harmless here; with a true maximum integer it would overflow in many languages.
- **Reading below 0.** Check `c ≤ t` before reading `fewest[t - c]`.
- **Forgetting -1.** On the "cannot make" preset (amount 7, coins 2 and 4) every odd total stays 999.

## When to use it

Use it for exact change, making a target from fixed step sizes, or any "fewest pieces that sum to exactly `t`" question. The linked problem, Change Machine, is exactly this.

## Related topics

- [Rod cutting (unbounded knapsack)](learn:dynamic-programming/rod-cutting): the same reuse of pieces, maximising a price instead.
- [0/1 knapsack](learn:dynamic-programming/knapsack-01): when each item can be used only once.
- [Breadth-first search](learn:graphs/bfs): totals as nodes and coins as edges; the fewest coins is the fewest edges from 0.
- [Activity selection](learn:greedy/activity-selection): when the greedy choice is safe, and how to argue it.
