Rechecking every course's prerequisites at the start of every term works, but there can be as many terms as courses. Which courses can become available in the next term? Only those that depend on something passed this term.
---
Keep, for each course, how many of its prerequisite rules are still not passed (a repeated rule counts each time), and for each course the list of courses it unlocks. When a course is passed, lower the count of every course it unlocks; a course whose count reaches 0 joins the next term.
---
Kahn's algorithm in layers: the first term is every course with count 0. For each term, sort it, record it, then go through its courses' unlock lists and collect the courses whose count drops to 0: that is the next term. Stop when a term is empty. If fewer than `n` courses were passed, some courses sit on a loop of rules (or depend on one): print `IMPOSSIBLE`.
