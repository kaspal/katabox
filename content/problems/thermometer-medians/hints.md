Sorting all readings again after every new one costs O(n² log n). The median only needs the one or two values in the middle.
---
Split the readings into a lower half and an upper half, where every value in the lower half is ≤ every value in the upper half. The median comes from the largest value of the lower half and the smallest of the upper half.
---
Keep the lower half in a max-heap and the upper half in a min-heap, with sizes equal or the lower half one larger. Insert the new reading into the right heap, then move one value across if the sizes drift apart. Each step is O(log n).
