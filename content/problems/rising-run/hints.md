Try a route where every checkpoint has the same height, such as `[2, 2]`. The answer should be 1.
---
The idea is sound: `tails[k]` is the smallest last height of a rising pick of length `k + 1`, and each height either extends `tails` or replaces one entry. Look at the binary search that chooses the position: what does it do when `h` is already in `tails`?
---
`bisect_right` returns the position after any equal values, so a repeated height lands past its twin and extends the run, which counts non-decreasing picks. Use `bisect_left` so an equal height replaces its twin instead (in TypeScript, change `tails[mid] <= h` to `tails[mid] < h`).
