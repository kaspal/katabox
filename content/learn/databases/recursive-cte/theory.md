## The idea

A plain query cannot follow a chain of unknown length: "who reports to whom, at any depth?" needs one join per level, and nobody knows in advance how many levels there are. A **recursive common table expression** solves that by letting a query read its own result:

```sql
WITH RECURSIVE chain(id, depth) AS (
  SELECT id, 0 FROM emp WHERE boss IS NULL                     -- the anchor
  UNION ALL
  SELECT e.id, c.depth + 1 FROM emp e JOIN chain c ON e.boss = c.id  -- the recursive part
)
SELECT depth FROM chain ORDER BY id
```

It runs in rounds, not as true recursion:

- The **anchor** runs once. Here it finds the employees with no boss and gives them depth 0.
- Each **round** runs the recursive part, but `chain` in it stands only for the rows found in the **previous** round, the **working table**. Joining them with `emp` finds the employees who report to them, one level deeper.
- The new rows are added to the result and become the next working table. When a round finds nothing, the query stops.

So the rows arrive level by level, exactly like a breadth-first search from the top of the tree.

### The default input

The array `[-1, 0, 0, 1, 1, 2]` gives each employee's boss: employee `i` reports to `a[i]`, and `-1` means no boss. Employee 0 is the top; 1 and 2 report to 0; 3 and 4 report to 1; 5 reports to 2.

Anchor: employee 0 has no boss, depth 0. Round 1 joins the working table `{0}` and finds 1 and 2, depth 1. Round 2 joins `{1, 2}` and finds 3 and 4 (boss 1) and 5 (boss 2), depth 2. Round 3 joins `{3, 4, 5}` and finds nobody, so the query stops after 3 rounds.

The output is `[0, 1, 1, 2, 2, 2]`: each employee's depth, by id.

## Steps

1. Anchor: give depth 0 to every employee whose boss is `-1`; they form the first working table.
2. Round: for each employee `b` in the working table, find every employee `e` with boss `b` and give `e` depth `depth[b] + 1`.
3. The employees found become the next working table.
4. Repeat until a round finds nobody, then return the depths by id.

## Cost

With `n` employees:

| Case | Cost |
|---|---|
| Best | `O(n)` with an index on `boss`: each employee is looked up once |
| Average | `O(n · rounds)`, scanning the table once each round |
| Worst | `O(n²)`: one long chain needs `n` rounds |
| Extra space | `O(n)` for the result and the working table |

The sample keeps it simple and scans every employee for each row of the working table, the quadratic version.

## Pitfalls

- **Cycles.** If 1 reports to 2 and 2 reports to 1, every round finds a row again and `UNION ALL` never stops. Guards: `UNION` instead of `UNION ALL` drops rows already in the result (this only helps when the repeated rows are identical, so not with a growing `depth`), or a condition such as `WHERE c.depth < 100` in the recursive part. The input here is a tree and never loops.
- **Joining the whole result each round.** The recursive part sees only the previous round; thinking it sees everything leads to wrong depths and duplicate rows.
- **Forgetting the anchor's filter.** Without `WHERE boss IS NULL`, every employee starts at depth 0.

## When to use it

Use a recursive query for hierarchies and chains stored as "parent" columns: org charts, folder trees, bills of materials, family trees, or every node reachable in a graph. The linked problems, Org Chart and Family Generations, walk exactly this kind of boss or parent list.

## Related topics

- Breadth-first search: the same level-by-level frontier.
- Depth-first search: another way to walk a tree from its root.
- Recursion and the call stack: real recursion, compared with these rounds.
- Join algorithms: the join each round performs.
