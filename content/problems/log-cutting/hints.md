Think about the **first** cut made inside a piece: it splits the piece into two parts that are then cut independently.
---
Sort the marks and add the two ends: positions `p[0] = 0, p[1..m], p[m+1] = length`. Let `cost[i][j]` be the cheapest way to make every cut strictly between `p[i]` and `p[j]` in the piece `p[i]..p[j]`.
---
`cost[i][j] = (p[j] − p[i]) + min over i < k < j of (cost[i][k] + cost[k][j])`, and 0 when `j = i + 1`. Fill it by increasing `j − i`.
