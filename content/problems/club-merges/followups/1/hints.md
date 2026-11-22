The base problem's union-find tells you which clubs merge. What extra number per club would answer this question?
---
Keep the size of every club at its root, and add the two sizes when two roots join. Can the biggest club ever shrink?
---
Clubs only grow, and only the club just formed can become the new biggest. Keep a running maximum, starting at 1, and compare it with the new club's size after each real merge; a merge inside one club leaves the answer as it was.
