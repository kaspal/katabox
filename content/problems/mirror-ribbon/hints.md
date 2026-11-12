You could build a cleaned copy of the text and compare it with its reverse. Can you avoid building any copy?
---
Read from both ends at once. What should happen when the character under one end is a space or a symbol?
---
Keep `i` at the start and `j` at the end. Skip characters that are not letters or digits on each side; then compare the two characters in lower case. A mismatch means `false`; otherwise move both inwards until they meet.
