Try a list with a single price, for example `["4.35"]`. Is the answer exactly 435?
---
The trouble starts the moment a price string becomes a number. Can a value like 4.35 be stored exactly as a floating-point number? What happens to a tiny error when you multiply by 100 and cut off the fraction?
---
Floats cannot hold most decimal prices exactly, so `4.35 * 100` comes out as 434.99999999999994 and `int` truncates it to 434. Do not use floats at all: remove the dot and read each price as a whole number of cents (`int(p.replace(".", ""))`), then add those integers.
