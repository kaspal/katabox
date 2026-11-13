The best poster is limited by its lowest panel. If you knew which panel is the lowest one under the poster, how wide could the poster be?
---
For panel `k` as the lowest, the poster stretches left and right until the first panel **lower** than `panels[k]` on each side. You need, for every panel, the nearest lower panel on both sides.
---
A stack of panel positions with increasing heights finds both at once: when panel `i` is lower than the top of the stack, pop it; the popped panel's poster ends just before `i` and starts just after the panel now on top of the stack.
