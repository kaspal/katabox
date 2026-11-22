Compare an input whose sum divides evenly by its length with one that does not, such as `[1, 2]`. Which one comes out wrong?
---
The sum is fine. Look closely at how it is divided by the count.
---
`//` is floor division in Python (and `Math.floor` in the TypeScript version), so the fractional part is thrown away. Use true division `/` and drop the floor.
