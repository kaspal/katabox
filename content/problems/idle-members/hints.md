You are looking for members for whom something does *not* exist: a visit in November. Which SQL construct asks "is there no such row?"
---
`WHERE NOT EXISTS (SELECT 1 FROM visits v WHERE v.member_id = m.id AND ...)` keeps a member only when the inner query finds nothing. The inner query must look only at November visits, not at every visit: a visit in October says nothing about November.
---
Dates written `YYYY-MM-DD` compare correctly as text, so `v.day BETWEEN '2026-11-01' AND '2026-11-30'` is November, both ends included. Select `m.id, m.name` and finish with `ORDER BY m.name, m.id`.
