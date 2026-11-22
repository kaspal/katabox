Recounting both bags after every change costs O(n) each time, up to 10¹⁰ steps in total. A change only touches two letters: the one taken out and the one put in.
---
Keep, for each of the 26 letters, its count in `a` minus its count in `b`. The bags are twins exactly when all 26 differences are 0.
---
Also keep how many letters have a nonzero difference, and keep `b` as an array so you know which letter a change removes. On each change, update the difference of the old tile and of the new one, and adjust the counter whenever a difference becomes zero or stops being zero. Answer `true` when the counter is 0.
