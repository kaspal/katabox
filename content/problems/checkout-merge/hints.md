The next customer of the merged line is always at the front of one of the lines. Which one?
---
Looking at every line's front for each customer costs k comparisons each time (k lines). Is there a structure that gives the smallest of k values quickly and lets you replace it?
---
Keep a min-heap of `(front ticket, line index)` for the non-empty lines. Pop the smallest, append that node to the result, and push the next node of the same line if there is one. Each customer is pushed and popped once.
