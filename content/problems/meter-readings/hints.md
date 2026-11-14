Read the whole log at once and split it into tokens instead of reading one line per call: with tens of thousands of lines, the cost of each read call adds up.
---
All three numbers can be kept up to date while you walk the readings once: a running sum, a counter of rises, and the length of the current run of equal readings. Never go back to the start of the log.
---
Keep `prev`, the previous reading. For each new reading `x`: add it to the sum; if `x > prev` count a rise; if `x == prev` the current run grows by 1, otherwise it restarts at 1; update the longest run. Start with `run = longest = 1` on the first reading.
