## Approach: a stack of folder names

Split the path at every `/`. The stack holds the folders from the top down to where you are:

- empty part or `.`: nothing changes;
- `..`: pop the last folder, unless the stack is empty (you are at the top);
- anything else: push it, even if it is made of dots (`...` is a name).

At the end, the clean path is `/` + the stack joined with `/` (just `/` for an empty stack).

### Walkthrough

`/photos/./2019//summer/../winter/` splits into `"", photos, ., 2019, "", summer, .., winter, ""`. The stack goes `[photos]`, `[photos, 2019]`, `[photos, 2019, summer]`, back to `[photos, 2019]`, then `[photos, 2019, winter]`.

**Complexity:** O(length) time and space.
