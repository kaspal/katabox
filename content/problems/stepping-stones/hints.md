You do not need the exact route: only how far the hiker could possibly get.
---
Walk the stones from left to right, keeping `reach`, the farthest stone reachable so far. Every stone up to `reach` is reachable.
---
At stone `i`: if `i > reach`, the hiker is stuck (false); otherwise `reach = max(reach, i + jumps[i])`. If you get through every stone, the last one is reachable.
