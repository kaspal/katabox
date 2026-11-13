Comparing the pattern at every position can cost `text length × pattern length`, for example when both are almost all `a`. After a partial match fails, what do you already know about the text?
---
The text you just matched **is** a prefix of the pattern. So after a mismatch you can continue from the longest prefix of the pattern that is also a suffix of what matched, instead of starting over.
---
The prefix function: `pi[i]` is the length of the longest proper prefix of `pattern[0..i]` that is also its suffix. Build it for the pattern, then scan the text keeping the current match length `k`, falling back with `k = pi[k − 1]` on mismatches. Each time `k` reaches the pattern length, count one and fall back.
