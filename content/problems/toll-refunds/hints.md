Dijkstra assumes a town's cost never improves once it is the cheapest unvisited one. With refunds that is false: a later road can make an earlier town cheaper.
---
Bellman–Ford: repeatedly relax every road (`cost[b] = min(cost[b], cost[a] + c)`). A cheapest route without loops uses at most n − 1 roads, so n − 1 rounds are enough.
---
After n − 1 rounds, try one more round: if any reachable road can still lower a cost, a negative loop is reachable, so return an empty list. Only relax roads that start from a reached town.
