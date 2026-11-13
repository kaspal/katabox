Deciding at each station whether to stop is hard to do on the spot. What if you could stop "in hindsight", once you know you need fuel?
---
Drive as far as the current fuel allows, remembering every station you passed. When you cannot go further, you must have stopped at one of them: which one helps most?
---
Keep a max-heap of the litres of passed stations. Whenever the next station or the target is beyond your reach, pop the largest and add it (one more stop). If the heap is empty, return −1.
