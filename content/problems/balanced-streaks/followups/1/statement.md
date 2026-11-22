The club still writes its season as a string of `W` (win), `L` (loss) and `D` (draw),
and a run of **consecutive** matches is still *balanced* when it has exactly as many
wins as losses (draws do not matter). This time the coach does not want to count the
balanced runs: return the **length of the longest** one.

A run made only of draws is balanced. If no run is balanced (for example `"WW"`),
return `0`.

### Example 1

```
results = "WWDLWLLW"
Output: 7
```

The run "WDLWLLW" (every match but the first) has three wins and three losses.

### Example 2

```
results = "WLLLD"
Output: 2
```

### Example 3

```
results = "WDDD"
Output: 3
```

"DDD" has no wins and no losses.

### Constraints

- 1 ≤ results.length ≤ 10⁵
- `results` contains only `W`, `L` and `D`
