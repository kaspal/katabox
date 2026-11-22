Running the base problem's search once per station would cost O(n · (n + links)): far
too slow for 10⁵ stations. Did the base search already learn more than it returned?
---
A breadth-first search from station `0` reaches the stations in order of their distance,
and it finds the fewest hops to every station it visits, not only to `n − 1`.
---
Run one breadth-first search from station `0` over both directions of every link, and
keep going until the queue is empty instead of stopping early. Record each station's
distance the first time you see it; stations never seen keep `-1`.
