Inputs where every time is different come out right. Try two runners with the same time.
---
Sorting by time works. Look at how ties between equal times are broken.
---
The first sort orders names with `reverse=True`, and the stable sort by time keeps that descending order for ties. Drop `reverse=True` (in TypeScript, the tie-break returns `1` and `-1` the wrong way round).
