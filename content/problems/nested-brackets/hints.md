When you meet a closing bracket, which opening bracket must it match?
---
It must match the most recent opening bracket that is still open. What structure gives you "the most recent one" quickly?
---
Push opening brackets on a stack. On a closing bracket, the stack must be non-empty and its top must be the same kind; pop it. At the end the stack must be empty.
