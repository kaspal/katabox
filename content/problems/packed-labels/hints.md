Unpacking is out of the question: one label can be billions of letters long. And walking a long label run by run for every check is still too slow when 20 000 checks hit a label with 60 000 runs.
---
Parse each label once into its runs. Position `k` falls in the first run whose **end** (the total of the counts up to and including that run) is at least `k`. Those ends only grow along the label.
---
Store, per label, the list of run ends (prefix sums of the counts) and the run letters. For a check, binary-search the first end ≥ `k` and print that run's letter. The ends stay below 2⁵³, so they are exact even in TypeScript.
