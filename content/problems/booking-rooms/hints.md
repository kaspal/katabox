The number of rooms you need is the largest number of meetings happening at the same moment. When can that number go up?
---
It only goes up when a meeting starts. Walk through time in order: every start takes a room, every end frees one.
---
Sort all start times and all end times separately. Sweep with one pointer on each list: if the next start is before the next end, a room is taken; otherwise a room is freed (at equal times, free first). The maximum number of rooms in use is the answer.
