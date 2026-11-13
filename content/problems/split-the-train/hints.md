First find how long the train is: the part lengths depend only on n and k.
---
With n wagons and k parts, every part gets n // k wagons, and the first n mod k parts get one extra.
---
Walk the list once more: for each part, keep its first node, step `size − 1` times to its last node, cut the link after it and move on. An empty part is `null`.
