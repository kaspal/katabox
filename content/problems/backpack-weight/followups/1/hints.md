You could treat every copy as its own item and run the base problem's table: that is up to
5000 items over 5000 capacities. It works, but can you get away with fewer "items"?
---
To allow any number of copies from 0 to `c`, you do not need `c` single copies. Bundles of
1, 2, 4, 8, ... copies, plus one bundle for whatever is left over, can add up to every
number from 0 to `c`.
---
Split each item's `counts[i]` copies into bundles of 1, 2, 4, ... copies and a final
remainder bundle. Treat each bundle as one 0/1 item with `k` times the weight and worth,
and run the base problem's table (capacities from high to low) over all the bundles.
