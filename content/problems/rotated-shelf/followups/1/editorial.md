## Idea

You could repeat the base problem's O(log n) search for every label, but it is simpler
to undo the move once. Binary search for the smallest number: if `shelf[mid]` is greater
than the last number, the start lies right of `mid`; otherwise it is at `mid` or to its
left. With the start `s`, index `k` of the sorted order sits at shelf position
`(k + s) mod n`, so an ordinary binary search over that virtual sorted array (or over a
rotated copy) answers each label; return `-1` when the number found is not the label.

## Complexity

O(n + q log n) time with a sorted copy, or O(log n + q log n) using the virtual view;
O(n) or O(1) extra memory besides the output, where `q` is the number of labels.
