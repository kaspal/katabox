For one user, which of their allowed requests can still matter at time `t`?
---
Only those after `t − window`. Because time only moves forward, a request that has dropped out of the window never comes back.
---
Keep, per user, a queue of the times of their allowed requests. At each operation, remove times `≤ time − window` from the front; the queue's length is the count. A request is allowed when the length is below `limit`, and then its time is added at the back.
