Adding the crews to every section of every repair is up to 10⁵ × 10⁵ steps. Can a repair be recorded with a constant amount of work?
---
Instead of the counts, record where they **change**: a repair adds `crews` starting at `a` and stops adding after `b`.
---
Keep `change[i]`: `change[a] += crews` and `change[b + 1] −= crews`. A running sum of `change` from the left gives each section's count.
