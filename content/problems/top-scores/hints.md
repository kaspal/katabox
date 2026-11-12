Sorting everything and taking the first `k` works. Can you avoid keeping more than `k` scores at a time?
---
Keep the best `k` scores seen so far. When a new score arrives, it only matters if it beats the worst of those.
---
A min-heap of size `k` gives you the worst kept score in O(1) and replaces it in O(log k). At the end, sort the `k` kept scores from highest to lowest.
