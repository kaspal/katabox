A table-tennis club writes down its team's season as a string: `W` for a win, `L` for a loss and `D` for a draw, one letter per match.

The coach calls a run of **consecutive** matches *balanced* when it has exactly as many wins as losses (draws do not matter). How many balanced runs does the season contain? A run is any non-empty stretch of consecutive matches; runs at different positions count separately, even if they read the same.

### Example 1

```
results = "WDL"
Output: 2
```

The balanced runs are "D" (no wins, no losses) and "WDL".

### Example 2

```
results = "WWLL"
Output: 2
```

"WL" in the middle and the whole season "WWLL".

### Example 3

```
results = "WLW"
Output: 2
```

"WL" and "LW".

### Constraints

- 1 ≤ results.length ≤ 10⁵
- `results` contains only `W`, `L` and `D`
