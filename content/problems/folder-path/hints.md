Split the path at `/` and handle the parts one by one. What do you need to remember about where you are?
---
Only the list of folders from the top down to the current one. Going into a folder adds one at the end; `..` removes the last one.
---
Use a stack: skip empty parts and `.`; on `..`, pop if the stack is not empty; otherwise push the name. Join the stack with `/` and put a `/` in front.
