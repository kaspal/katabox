## Approach: one pass over the runs

Keep the start index of the current run. Move `i` forward while `text[i]` equals the run's letter. When it changes (or the text ends), the run is `text[start:i]`: emit its letter, and its length `i - start` when that is at least 2. Then start the next run at `i`.

### Walkthrough

`text = "aaabcc"`:

| run | length | emitted | label so far |
| --- | --- | --- | --- |
| aaa | 3 | `a3` | `a3` |
| b | 1 | `b` | `a3b` |
| cc | 2 | `c2` | `a3bc2` |

Lengths can have several digits: a run of 100000 `z` becomes `z100000`.

**Complexity:** O(n) time, and O(n) for the output. Building the label with a list (or a string builder) keeps it linear in every language.
