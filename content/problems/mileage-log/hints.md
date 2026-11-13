Adding the days of every trip one by one works, but 10⁵ trips of 10⁵ days is far too much. What could you compute once and reuse?
---
If you knew the total from day 0 up to any day, could you get the total of days a to b from two such numbers?
---
Build `prefix[i]` = total of days 0 to i − 1 (with `prefix[0] = 0`). A trip [a, b] is `prefix[b + 1] − prefix[a]`.
