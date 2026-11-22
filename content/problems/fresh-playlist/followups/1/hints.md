Rerunning the base problem's window inside every request costs up to 10⁵ steps per
request. Run the window once over the whole log instead: for every play `r`, where does
the longest fresh stretch ending at `r` start? Call it `start[r]`.
---
`start[r]` never decreases. For a request `[a, b]`, the plays `r` with `start[r] < a` are
cut off at `a`: their best stretch inside is `a..r`, longest for the last such `r`. The
others keep their whole stretch, of length `r − start[r] + 1`.
---
Binary search the first `r0` in `[a, b]` with `start[r0] ≥ a`. The answer is the larger of
`r0 − a` (the stretch `a..r0 − 1`) and the maximum of `r − start[r] + 1` over `r0..b`, which
a sparse table answers in O(1).
