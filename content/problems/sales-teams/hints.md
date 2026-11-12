Adding each employee's sales to every boss above them is correct, but in a long chain of command that is about n²/2 additions.
---
A team total is the employee's own sales plus the team totals of the people they lead directly. So if every direct report's total is known first, a boss's total takes one addition per report.
---
List the employees from the top down (a breadth-first walk from the owner over children lists). Then go through that list **backwards**, adding each employee's total into their boss's: everyone is finished before their boss is used.
