A music player keeps its playlist as a linked list of song ids, `playlist`, from the first song to the last. Its **Replay last** button takes the last song of the playlist and moves it to the front.

You press the button `k` times. Return the playlist after the presses. An empty playlist stays empty.

Lists are written as their values in order, so `[4, 8, 15]` is the list 4 → 8 → 15, and `[]` is the empty list.

### Example 1

```
playlist = [4, 8, 15, 16, 23], k = 2
Output: [16, 23, 4, 8, 15]
```

The first press moves 23 to the front, the second moves 16.

### Example 2

```
playlist = [7, 9], k = 5
Output: [9, 7]
```

Every two presses bring this playlist back to how it started.

### Example 3

```
playlist = [], k = 3
Output: []
```

### Constraints

- 0 ≤ number of songs ≤ 10⁵
- 1 ≤ song id ≤ 10⁶
- 0 ≤ k ≤ 10⁹
