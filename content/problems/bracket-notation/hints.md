The text of a node contains the texts of its children: a recursive function fits.
---
Write the value, then decide on the left part: it is needed when there is a left child, **or** when there is a right child (as `()`).
---
`text(node) = str(val) + ("(" + text(left) + ")" if left or right else "") + ("(" + text(right) + ")" if right else "")`. Build the string in a list and join it once for long trees.
