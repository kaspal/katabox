Trying every group of tokens means up to 2¹⁰⁰ groups. The base problem's table over
amounts is still the way, but it let one token be used again and again. What has to
change so each token is used at most once?
---
Add the tokens one by one. After processing some of them, `best[a]` is the fewest of
those tokens that make `a`. The new token either stays out or is used once on top of a
table that did not contain it yet.
---
For each token `t`, go through the amounts **downwards**, from `amount` to `t`, and
improve `best[a]` with `best[a − t] + 1`. Going down means `best[a − t]` has not been
updated with `t` yet, so `t` is counted at most once. Start with `best[0] = 0` and every
other amount unreachable.
