Each row on its own is sorted. What is the simplest search that uses that?
---
A binary search in every row is O(rows · log columns). Can one comparison rule out a whole row or a whole column instead?
---
Start at the top-right seat. If it is bigger than the ticket, everything below it in its column is bigger too: drop the column (move left). If it is smaller, everything to its left in its row is smaller: drop the row (move down). Equal means found.
