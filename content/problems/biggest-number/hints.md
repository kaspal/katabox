Sorting the cards as numbers (76 < 760) or as text ("76" < "760") both go wrong somewhere. Look at just two cards: which should come first?
---
Card `a` should go before card `b` exactly when the text `a + b` is bigger than `b + a` (both have the same length, so compare them as strings).
---
Sort with that comparison, join the texts, and turn an all-zero result into "0".
