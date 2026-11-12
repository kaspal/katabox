The heaviest hiker must cross on some trip. Who is the best companion for them, if anyone?
---
If the heaviest hiker cannot share with the lightest one, they cannot share with anyone. If they can, pairing them with the lightest never hurts.
---
Sort the weights. Keep `lo` at the lightest and `hi` at the heaviest: every trip takes `hi` (then `hi -= 1`), and also `lo` when `weights[lo] + weights[hi] ≤ limit` (then `lo += 1`). Count the trips until the pointers cross.
