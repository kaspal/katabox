Splitting on spaces is not enough: in `rain,rain!sun` there are three words and no space. Read everything after the first line and pull out every run of letters `a`-`z`, for example with the regular expression `[a-z]+`.
---
Count the words in a hash map from word to count. The number of text lines does not matter: one read of the whole input gives you all of them.
---
Sort the (word, count) pairs by count descending, then by word ascending, and print the first `K` (or all of them if there are fewer). In Python the key `(-count, word)` does both at once.
