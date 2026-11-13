A small web service protects itself with a throttle: each user may have at most `limit` **allowed** requests in any stretch of `window` seconds. Precisely, a request from user `u` at time `t` is allowed when fewer than `limit` of `u`'s earlier requests were allowed at times `t'` with `t − window < t' ≤ t`. Refused requests do not count.

Process the operations in order, where `ops[i]` is the name and `args[i]` the numbers:

- `"request"`, args `[user, time]`: **report** `1` if the request is allowed, `0` if it is refused;
- `"count"`, args `[user, time]`: **report** how many of the user's requests were allowed at times `t'` with `time − window < t' ≤ time`.

Times never decrease from one operation to the next. Return the reports in order.

### Example 1

```
limit = 2, window = 10, ops = ["request", "request", "request", "request", "request"], args = [[1, 1], [1, 3], [1, 5], [1, 11], [2, 11]]
Output: [1, 1, 0, 1, 1]
```

At time 5, user 1 already has 2 allowed requests in (−5, 5]. At time 11, the window is (1, 11], which holds only the request at time 3.

### Example 2

```
limit = 1, window = 5, ops = ["request", "count", "request", "count"], args = [[7, 0], [7, 4], [7, 5], [7, 5]]
Output: [1, 1, 1, 1]
```

### Example 3

```
limit = 3, window = 60, ops = ["count"], args = [[3, 100]]
Output: [0]
```

### Constraints

- 1 ≤ limit ≤ 1000, 1 ≤ window ≤ 10⁹
- 1 ≤ ops.length ≤ 10⁵, args.length = ops.length
- 1 ≤ user ≤ 10⁹, 0 ≤ time ≤ 10⁹, and times never decrease
