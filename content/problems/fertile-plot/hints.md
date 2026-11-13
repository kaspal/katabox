With a single row, this is the classic largest-sum stretch of an array. Can you turn a rectangle problem into a one-row problem?
---
Fix the top and bottom rows of the plot. Adding up each column between them gives one array, and the best stretch of that array (Kadane's algorithm, O(C)) is the best plot with exactly those two rows.
---
Loop over the top row; then move the bottom row down one at a time, adding that row into running column sums, and run Kadane after each step: O(R²·C) in all. Put the shorter side in the outer loops: when R > C, transpose the grid first, so a 4000 × 20 field costs about 20²·4000 steps instead of 4000²·20.
