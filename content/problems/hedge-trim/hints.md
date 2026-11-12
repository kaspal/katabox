A branch can only be judged after its sub-branches, so the order of the walk matters. Which traversal visits the children before their parent?
---
For each branch you need the total of what remains below it after the cuts there. Can each call return that total along with the trimmed sub-branch?
---
Post-order: trim the left and right children first, each returning (what remains, its total). The branch's total is its value plus those totals. If it is negative, return (nothing, 0); otherwise attach the trimmed children and return (the branch, the total).
