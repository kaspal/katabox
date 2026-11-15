Walking up from `b` through its bosses answers one question, but the chart can be a single line of 10⁵ people and there are up to 10⁵ questions. What could you number once, before the questions, so that each answer is a comparison?
---
Visit the chart depth first from the director and give each employee an entry number when you first reach them. Everyone below `x` is visited after `x` and before the search leaves `x`'s team, so their numbers form one unbroken range.
---
Record `first[x]` (the entry number of `x`) and `last[x]` (the largest entry number in `x`'s team). Then `a` is above `b` exactly when `a ≠ b` and `first[a] ≤ first[b] ≤ last[a]`. Run the search with an explicit stack, not recursion: the chart can be 10⁵ levels deep.
