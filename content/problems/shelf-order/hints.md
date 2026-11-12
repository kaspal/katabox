Sorting needs only a way to compare two labels; any good sorting algorithm does the rest. Write the comparison following the rules, chunk by chunk.
---
Digit chunks can have up to 20 digits, more than a 64-bit integer holds. Compare them as text instead: remove the leading zeros, then the shorter one is the smaller number, and equal lengths compare digit by digit.
---
In Python, a sort key does all of this: turn each label into a tuple with one entry per chunk, `(0, len(n), n)` for a digit chunk `n` without leading zeros and `(1, s)` for a letter chunk `s`. Tuples compare entry by entry and a shorter tuple that matches comes first. Use a **stable** sort so equal labels keep their order.
