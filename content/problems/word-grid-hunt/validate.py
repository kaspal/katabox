import string


def validate(inp):
    grid, words = inp["grid"], inp["words"]
    if not isinstance(grid, list) or not all(isinstance(r, str) for r in grid):
        return "grid is a list of strings"
    if not isinstance(words, list) or not all(isinstance(w, str) for w in words):
        return "words is a list of strings"
    if not 1 <= len(grid) <= 10 or not all(1 <= len(r) <= 10 and len(r) == len(grid[0]) for r in grid):
        return "1 ≤ rows, columns ≤ 10, all rows the same length"
    if not 1 <= len(words) <= 3 * 10**4:
        return "1 ≤ words.length ≤ 3·10⁴"
    if not all(1 <= len(w) <= 8 for w in words):
        return "1 ≤ words[i].length ≤ 8"
    if not all(ch in string.ascii_lowercase for s in grid + words for ch in s):
        return "The grid and the words use lowercase English letters only."
    return None
