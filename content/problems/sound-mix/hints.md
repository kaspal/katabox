Checking every stretch is about `n²/2` stretches, far too many for `n = 10⁵`. "Exactly `K`" is awkward to count directly; is "at most `K`" any easier?
---
If a stretch has at most `m` different sources, so does every shorter stretch inside it. So for each end second, the good starts form one block that ends at the end second, and its left edge only moves right as the end moves right: a sliding window. Then **exactly `K` = at most `K` − at most `K − 1`**.
---
`atMost(m)`: keep a count per id in a hash map (ids go up to 10⁹, so not an array). For each `right`, add `ids[right]`; while the map holds more than `m` ids, remove `ids[left]` (delete it when its count drops to 0) and move `left` on; then add `right − left + 1` to the total. Print `atMost(K) − atMost(K − 1)`, with `atMost(0) = 0`.
