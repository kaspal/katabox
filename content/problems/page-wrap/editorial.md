## Approach: fill each line greedily

The rules describe a single pass: keep the current line's words and length, and for each word decide whether it joins the line or starts a new one.

```
lines = []; current = [words[0]]; length = len(words[0])
for w in words[1:]:
    if length + 1 + len(w) <= width:      # fits after one space
        current.append(w); length += 1 + len(w)
    else:
        lines.append(" ".join(current))
        current = [w]; length = len(w)    # a long word still starts a line
lines.append(" ".join(current))
```

A word longer than `width` never fits after another word, so it starts a line, and the next word cannot fit after it either: it ends up alone, as required. Joining the words of a line once at the end keeps the work linear.

### Walkthrough

Width 10: "the" (3), "quick" fits (9), "brown" does not (15) → line "the quick". "brown" (5), "fox" fits (9), "jumps" does not (15) → "brown fox". The last line is "jumps".

**Complexity:** O(total characters) time and output size.
