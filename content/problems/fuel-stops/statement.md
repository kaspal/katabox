A delivery van must drive `target` kilometres along a straight road. It starts with `fuel` litres and burns exactly one litre per kilometre; the tank is big enough for any amount. Along the road are fuel stations, `stations[i] = [position, litres]`: stopping at a station adds all of its `litres` to the tank.

Return the fewest stops needed to reach the target, or `-1` if it cannot be reached. Reaching a station or the target with an empty tank still counts as reaching it.

### Example 1

```
target = 50, fuel = 15, stations = [[10, 20], [20, 5], [30, 30], [40, 10]]
Output: 2
```

Stop at 10 (5 litres left + 20 = 25, enough to reach 30), stop at 30 for 30 more, and finish. One stop is not enough: with 15 litres only the station at 10 is in reach, and it gets the van to 35.

### Example 2

```
target = 30, fuel = 30, stations = [[10, 5]]
Output: 0
```

### Example 3

```
target = 40, fuel = 10, stations = [[10, 10], [25, 30]]
Output: -1
```

After stopping at 10 the van has 10 litres, not enough for the 15 km to the next station.

### Constraints

- 1 ≤ target, fuel ≤ 10⁹
- 0 ≤ stations.length ≤ 10⁵
- 0 < position < target, positions strictly increasing
- 1 ≤ litres ≤ 10⁹
