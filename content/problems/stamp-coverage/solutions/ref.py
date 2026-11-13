def busiest_cells(rows: int, cols: int, stamps: list[list[int]]) -> list[int]:
    d = [[0] * (cols + 1) for _ in range(rows + 1)]
    for t, l, b, r in stamps:
        d[t][l] += 1
        d[t][r + 1] -= 1
        d[b + 1][l] -= 1
        d[b + 1][r + 1] += 1
    most, cells = 0, 0
    above = [0] * cols
    for i in range(rows):
        row, cur, run = d[i], [0] * cols, 0
        for j in range(cols):
            run += row[j]
            cur[j] = above[j] + run
        top = max(cur)
        if top > most:
            most, cells = top, cur.count(top)
        elif top == most:
            cells += cur.count(top)
        above = cur
    return [most, cells]
