## Approach: depth-first entry numbers and team ranges

The obvious answer walks up from `b`, boss by boss, until it meets `a` or passes the director. That costs the depth of `b` per question. On a chart that is one line of 10⁵ people, with 2·10⁴ questions about the top and the bottom of the line, it is about 10⁹ steps. Instead, number the chart once so that every question becomes two comparisons.

**Entry numbers.** Build each employee's team (the list of people whose boss they are), then visit the chart depth first from the director. Give each employee an entry number `first[x]` when the search reaches them, and when the search has finished with everyone below `x`, record `last[x]`, the largest entry number handed out so far. Because the search enters `x` and then visits everyone below `x` before it leaves, the people below `x` get exactly the numbers `first[x] + 1 .. last[x]`.

For Example 1 (bosses `1 1 2 2 4`), visiting the last team member first:

```
employee   1  3  2  5  4  6
first      1  2  3  4  5  6
last       6  2  6  4  6  6
```

`2`'s range is `3..6`, which holds `5`, `4` and `6`: exactly the people below `2`.

**A question.** `a` is above `b` exactly when `a ≠ b` and

```
first[a] ≤ first[b] ≤ last[a]
```

**Why it is correct:** if `b` is below `a`, the search reaches `b` while it is inside `a`'s team, so `first[b]` lies in `a`'s range. If `b` is not below `a`, the search reaches `b` either before entering `a` or after leaving `a`'s team for good, so `first[b]` is outside the range. The test `a ≠ b` removes the case `first[b] = first[a]`.

**No recursion.** The natural recursive search goes as deep as the chart, and a chart can be 10⁵ levels deep: Python stops at its recursion limit (1000 by default), and TypeScript, PHP and Rust can overflow their call stacks. Use an explicit stack of `(employee, done)` pairs: popping `(x, false)` gives `x` its entry number, pushes `(x, true)` and then `x`'s team; popping `(x, true)` sets `last[x]`, because by then everyone pushed after it has been visited.

**Do not number by employee order.** A boss may have a larger number than the employee, so you cannot fill depths or ranges in a single pass from `2` to `n`; the search handles any numbering.

**Complexity:** O(n) to build the teams and run the search, O(1) per question: O(n + Q) time and O(n) memory. Every number fits easily in a 32-bit integer.

**Fast input and output:** read all tokens at once (the empty second line when `n` is 1 then simply contributes no tokens), and print all answers in one write.
