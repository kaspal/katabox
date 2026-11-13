def find_words(grid: list[str], words: list[str]) -> list[str]:
    trie = {}
    for w in words:
        node = trie
        for ch in w:
            node = node.setdefault(ch, {})
        node["$"] = w
    rows, cols = len(grid), len(grid[0])
    used = [[False] * cols for _ in range(rows)]
    found = set()

    def dfs(r, c, node):
        child = node.get(grid[r][c])
        if child is None:
            return
        if "$" in child:
            found.add(child["$"])
        used[r][c] = True
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols and not used[nr][nc]:
                dfs(nr, nc, child)
        used[r][c] = False

    for r in range(rows):
        for c in range(cols):
            dfs(r, c, trie)
    return sorted(found)
