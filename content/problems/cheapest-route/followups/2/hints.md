Making the most expensive road of the plain cheapest route free is not always best: a
route that is worse without the pass can be better with it. What else must you know when
you arrive at a town?
---
At every town, the courier is in one of two situations: the pass is still unused, or it
has been spent. Treat each (town, situation) pair as its own node.
---
Run Dijkstra on 2·n states. From an unused state, a road leads to the next town's unused
state for its toll, or to its spent state for 0. From a spent state, roads cost their
toll. The answer is the cheaper of the two states at `goal`.
