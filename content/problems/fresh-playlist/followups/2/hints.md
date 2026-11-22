Removing plays from either end of a stretch can only lower its number of different songs. Does a sliding window still apply?
---
Keep a map from song to how many times it appears in the window. The number of different songs is the number of songs with a count above 0.
---
Add `plays[end]`; while the map holds more than `k` songs, lower the count of `plays[start]`, delete the song from the map when its count reaches 0, and move `start` right. Keep the largest window you see.
