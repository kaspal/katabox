A queue per book handles `"hold"` and `"return"` in O(1). The trouble is `"cancel"`: removing someone from the middle of a list costs time proportional to the line.
---
Do not remove cancelled members right away. Remember who is still really waiting (a hash map keyed by book and member), and when a copy comes back, pop members from the front and **skip** those who are no longer waiting.
---
Careful with members who cancel and hold again: their old entry is still somewhere in the queue. Give every hold a unique ticket number, store it both in the queue and in the map, and accept a queue entry only if its ticket is the member's current one.
