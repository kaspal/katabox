Read the text from left to right and keep track of how many `(` are still waiting for their `)`. What does a `)` mean when none are waiting?
---
A `)` with nothing open can only be fixed by inserting a `(` somewhere before it. Every `(` still open at the end needs a `)` inserted after it.
---
Keep two counters: brackets still open and insertions. On `(` add one to the open count; on `)` take one away if it is positive, otherwise count one insertion. The answer is the insertions plus whatever is still open at the end.
