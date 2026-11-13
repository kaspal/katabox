def busiest_cells(rows: int, cols: int, stamps: list[list[int]]) -> list[int]:
    # Correct but O(stamps · rows · cols): ink every cell of every stamp.
    ink = [[0] * cols for _ in range(rows)]
    for t, l, b, r in stamps:
        for i in range(t, b + 1):
            row = ink[i]
            for j in range(l, r + 1):
                row[j] += 1
    cells = [v for row in ink for v in row]
    most = max(cells)
    return [most, cells.count(most)]
