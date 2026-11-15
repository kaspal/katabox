## Approach: Kahn's algorithm, one term at a time

Think of the courses as a directed graph with an edge `a → b` for each rule. A course is taken in the term right after its **latest** prerequisite is passed, or in term 1 if it has none.

**Counts and unlock lists.** For every rule `a b`, append `b` to `unlocks[a]` and add 1 to `missing[b]`. A repeated rule is simply counted twice in both places, so it stays consistent.

**Terms in layers.**

1. The first term is every course with `missing = 0`.
2. For the current term: sort it, record it, and for each of its courses `a`, lower `missing[b]` for every `b` in `unlocks[a]`. A course whose count drops to 0 has just had its last prerequisite passed, so it belongs to the **next** term.
3. Repeat until a term comes out empty.

**Worked example** (Example 1). `missing` is 0 for 1, 2 and 4: term 1 is `1 2 4`. Passing them leaves course 3 at 0 (both 1 and 2 are done), course 5 at 1 (waiting for 3) and course 6 at 1 (waiting for 5). Term 2 is `3`, which frees 5; term 3 is `5`, which frees 6; term 4 is `6`. Four terms.

**Why it is correct:** a course's count reaches 0 exactly while its last prerequisite's term is processed, so it lands in the term right after that one, which is what the student does. If the rules contain a loop, no course on the loop ever reaches 0 (each waits for another one on the loop), and neither does any course that depends on the loop. So if fewer than `n` courses were passed, print `IMPOSSIBLE`; if there is no loop, every course is reached, since some course with count 0 always remains among those not yet passed.

**Complexity:** each course is placed once and each rule is followed once: O(n + m), plus sorting the terms, O(n log n) in all. Memory O(n + m).

**Why the obvious way is too slow:** rescanning every course and its rules at the start of each term costs O(n + m) per term, and a hidden chain of courses can force `n` terms of one course each. With `n = 3·10⁴` and `m = 4·10⁴` that is around 2·10⁹ steps.

**Practical notes:** read all tokens at once and print the plan in one write. The method uses a plain list per term and no recursion, so a 3·10⁴-term chain is no problem (a recursive depth-first search would hit Python's recursion limit). In TypeScript, sort each term with `(x, y) => x - y`, not the default text order (which puts `10` before `9`).
