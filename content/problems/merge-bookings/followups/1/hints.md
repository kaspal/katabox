Comparing every booking of one room with every booking of the other is O(n · m), and the
overlaps inside each sheet make the pieces messy. Can you tidy each sheet first?
---
Combine each sheet on its own, exactly as in the base problem. Now each room is a sorted
list of ranges that share no moment. Two ranges `[s1, e1]` and `[s2, e2]` overlap in
`[max(s1, s2), min(e1, e2)]` when that start is at most that end.
---
Walk both combined lists with one pointer each. Record the overlap of the two current
ranges if it is not empty, then move past the range that ends first (either one if they
end together).
