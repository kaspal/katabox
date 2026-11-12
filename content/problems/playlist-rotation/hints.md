Pressing the button as many times as the playlist has songs changes nothing. How many of the `k` presses really matter?
---
After `r = k mod n` presses (n songs), the last `r` songs come first, in their order, followed by the others. Which song ends up last, and which one first?
---
Walk the list once to count `n` and find the tail. The new last song is the one `n − r − 1` steps after the head; the song after it becomes the new head. Cut the link there and hang the old head after the old tail.
