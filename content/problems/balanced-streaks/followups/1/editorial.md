## Idea

As in the base problem, turn the season into a running score (+1 per win, −1 per loss,
draws leave it unchanged). The run from match `a + 1` to match `b` is balanced exactly
when the score after `b` equals the score after `a`. Counting needed how many times each
score appeared; the longest run needs only the **first** time. Store `first[0] = −1`,
then at each match `i` either record `first[score] = i` (new score) or try
`i − first[score]` as the best length.

## Complexity

O(n) time and O(n) extra memory (the score stays between −n and n, so an array of size
2n + 1 works as well as a hash map).
