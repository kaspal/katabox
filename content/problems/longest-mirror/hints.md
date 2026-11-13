Checking every piece separately costs O(n) per piece and O(n³) in total. Every mirrored piece has a middle. What happens if you start from the middle?
---
From a middle, extend one letter to the left and one to the right while they are equal. A middle is either a letter (odd length) or the gap between two letters (even length): 2n − 1 middles in all.
---
Expand around each of the 2n − 1 middles in order from left to right, keeping the longest piece found; replace it only by a **strictly** longer one, so ties keep the piece that starts first.
