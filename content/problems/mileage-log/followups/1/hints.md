The base problem's prefix sums count every day, but here each question has its own
threshold `x`. Counting day by day is up to 10⁵ steps per question. What if all questions
had the same `x`?
---
With one threshold, mark the long days with 1 and the others with 0, and prefix sums
answer every range. Different thresholds just mean the marks change. If you answer the
questions from the largest `x` down, each day only ever switches from 0 to 1, once.
---
Sort the days by kilometres and the questions by `x`, both descending. Walk the questions,
first switching on every day with at least `x` km in a Fenwick tree (a 1 at its position),
then answer with two prefix sums: `sum(b) − sum(a − 1)`. Store each answer at its
question's original index.
