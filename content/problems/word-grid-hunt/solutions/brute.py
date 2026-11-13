def find_words(grid: list[str], words: list[str]) -> list[str]:
    # Obviously correct: for each distinct word, try every starting cell with a plain DFS.
    rows, cols = len(grid), len(grid[0])

    def spell(word, i, r, c, used):
        if not (0 <= r < rows and 0 <= c < cols) or (r, c) in used or grid[r][c] != word[i]:
            return False
        if i == len(word) - 1:
            return True
        used.add((r, c))
        ok = any(spell(word, i + 1, r + dr, c + dc, used) for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)))
        used.discard((r, c))
        return ok

    return sorted(w for w in set(words) if any(spell(w, 0, r, c, set()) for r in range(rows) for c in range(cols)))
