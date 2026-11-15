Each team keeps its own row, but its place depends on the other teams of its division. Which kind of function computes a value per row from a group of rows, without collapsing them?
---
A ranking window function over `PARTITION BY division ORDER BY points DESC` numbers each division from the top. SQLite has three: `ROW_NUMBER` never shares a number, `RANK` shares on ties but then skips (1, 1, 3), and `DENSE_RANK` shares without skipping (1, 1, 2).
---
Select `division, name, points, DENSE_RANK() OVER (PARTITION BY division ORDER BY points DESC) AS place` from `teams` and finish with `ORDER BY division, place, name, id`.
