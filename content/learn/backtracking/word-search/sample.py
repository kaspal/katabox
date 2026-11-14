DIRS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def find_word(rows, cols, letters, word):
    path = []

    def walk(r, c, k):
        if letters[r * cols + c] != word[k] or (r, c) in path:
            return False
        path.append((r, c))  # @step step
        if k == len(word) - 1:
            return True
        for dr, dc in DIRS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and walk(nr, nc, k + 1):
                return True
        path.pop()  # @step back
        return False

    for r in range(rows):
        for c in range(cols):
            if walk(r, c, 0):
                return [1] + [x for cell in path for x in cell]
    return [0]
