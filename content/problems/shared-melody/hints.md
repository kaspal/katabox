Look at the last notes of both tunes. Either they match and can end the shared melody, or one of them is not part of it.
---
Let `L[i][j]` be the answer for the first `i` notes of `first` and the first `j` notes of `second`.
---
If `first[i − 1] == second[j − 1]`, `L[i][j] = L[i − 1][j − 1] + 1`; otherwise `L[i][j] = max(L[i − 1][j], L[i][j − 1])`. Fill the table row by row; keeping only two rows is enough.
