The answer is one more than the largest number of steps from anybody up to the founder. Walking up from every person separately is correct, but on a long single line of descendants it takes about n²/2 steps.
---
Turn the parent array around: for every person, list their children. Then the tree can be explored **downwards** from the founder.
---
Go generation by generation (breadth-first): start with the founder, then all their children, then all of their children, … Count how many rounds there are until no one is left.
