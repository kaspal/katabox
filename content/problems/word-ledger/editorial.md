## Approach: extract the words, count them, sort the counts

**Reading free text.** Only the first line has a fixed meaning. After it comes text of unknown length, so there is no count to loop on: read the whole input at once, take the first line as `K` and treat the rest as one string. Words are not only separated by spaces (`rain,rain!sun`), so splitting on whitespace would leave punctuation glued to the words. Extract the words directly instead: every maximal run of `a`-`z`, which a regular expression `[a-z]+` finds in one call. Line breaks and empty lines need no special care.

**Counting.** A hash map from word to count, updated once per word.

**Ranking.** Sort the distinct words by count descending and, among equal counts, alphabetically; then print the first `K`. Python's `sorted` with the key `(-count, word)` expresses both rules; in other languages compare the counts first and the words second. Compare words by their characters (`<` on strings in TypeScript, `strcmp` in PHP, `Ord` on `&str` in Rust), not with a locale-aware comparison.

**Why it is correct:** every word is counted exactly once, and the sort order is exactly the requested one, so the first `K` entries are the answer (or all entries when there are fewer than `K`).

**Complexity:** O(L + D log D) time, where L is the length of the text and D the number of distinct words; O(L) memory.

**Fast input in each language:** `sys.stdin.buffer.read()` with `re.findall(rb"[a-z]+", ...)` and a `Counter` (Python), `readFileSync(0, "utf8")` with `match(/[a-z]+/g)` and a `Map` (TypeScript), `stream_get_contents(STDIN)` with `preg_match_all` and `array_count_values` (PHP), `read_to_string`, `split(|c: char| !c.is_ascii_lowercase())` and a `HashMap` (Rust).
