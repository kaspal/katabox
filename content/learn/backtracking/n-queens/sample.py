def solve_queens(n):
    cols, found = [], []

    def safe(c):
        r = len(cols)
        return all(
            c != cc and abs(c - cc) != r - rr for rr, cc in enumerate(cols)
        )

    def place():
        if len(cols) == n:
            found.append(list(cols))  # @step solution
            return
        for c in range(n):
            if safe(c):
                cols.append(c)  # @step place
                place()
                cols.pop()  # @step remove

    place()
    return [len(found)] + (found[0] if found else [])
