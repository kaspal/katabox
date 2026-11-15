First build the full list the site would show: drop the hidden listings and sort the rest. Then think about how many listings come before page 3.
---
`WHERE hidden = 0 ORDER BY rating DESC, title, id` gives the visible listings in page order. Pages 1 and 2 hold 20 listings between them, so page 3 starts after skipping 20, not 30. SQL can skip rows and cap how many it returns after the sort.
---
Add `LIMIT 10 OFFSET 20` after the `ORDER BY`: skip the first 20 rows, then return at most 10. When fewer than 21 listings are visible it simply returns no rows.
